use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ReserveRequest {
    user_id: Uuid,
}

#[derive(Serialize)]
struct ReservationResponse {
    reservation_id: Uuid,
    seat_id: Uuid,
}

#[derive(Serialize)]
struct WaitlistResponse {
    waitlist_id: Uuid,
}

#[derive(Deserialize)]
pub struct PaymentRequest {
    user_id: Uuid,
}

#[derive(Serialize)]
struct PaymentResponse {
    reservation_id: Uuid,
    seat_id: Uuid,
}

pub async fn reserve(
    State(pool): State<PgPool>,
    Path(event_id): Path<Uuid>,
    Json(body): Json<ReserveRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let event_exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM events WHERE id = $1)")
            .bind(event_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !event_exists {
        return Err(StatusCode::NOT_FOUND);
    }

    let seat_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM seats
         WHERE event_id = $1 AND status = 'available'
         LIMIT 1
         FOR UPDATE SKIP LOCKED",
    )
    .bind(event_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match seat_id {
        Some(seat_id) => {
            sqlx::query(
                "UPDATE seats
                 SET status = 'held',
                     held_until = now() + interval '15 minutes',
                     held_by_user_id = $1
                 WHERE id = $2",
            )
            .bind(body.user_id)
            .bind(seat_id)
            .execute(&mut *tx)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let reservation_id: Uuid = sqlx::query_scalar(
                "INSERT INTO reservations (seat_id, user_id) VALUES ($1, $2) RETURNING id",
            )
            .bind(seat_id)
            .bind(body.user_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            tx.commit()
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            Ok((
                StatusCode::CREATED,
                Json(ReservationResponse {
                    reservation_id,
                    seat_id,
                }),
            )
                .into_response())
        }
        None => {
            let waitlist_id: Uuid = sqlx::query_scalar(
                "INSERT INTO waitlist (event_id, user_id) VALUES ($1, $2) RETURNING id",
            )
            .bind(event_id)
            .bind(body.user_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            tx.commit()
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            Ok((StatusCode::ACCEPTED, Json(WaitlistResponse { waitlist_id })).into_response())
        }
    }
}

pub async fn pay(
    State(pool): State<PgPool>,
    Path(reservation_id): Path<Uuid>,
    Json(body): Json<PaymentRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let received_at = OffsetDateTime::now_utc();

    let mut tx = pool
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let row: Option<(Uuid, String, Option<Uuid>, bool)> = sqlx::query_as(
        "SELECT s.id, s.status, s.held_by_user_id, s.held_until < $2 AS expired
         FROM reservations r
         JOIN seats s ON s.id = r.seat_id
         WHERE r.id = $1
         FOR UPDATE OF s",
    )
    .bind(reservation_id)
    .bind(received_at)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let Some((seat_id, status, held_by, expired)) = row else {
        return Err(StatusCode::NOT_FOUND);
    };

    if status != "held" || held_by != Some(body.user_id) {
        return Err(StatusCode::CONFLICT);
    }

    if expired {
        return Err(StatusCode::FORBIDDEN);
    }

    sqlx::query(
        "UPDATE seats
         SET status = 'sold',
             held_until = NULL,
             held_by_user_id = NULL
         WHERE id = $1",
    )
    .bind(seat_id)
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::OK,
        Json(PaymentResponse {
            reservation_id,
            seat_id,
        }),
    )
        .into_response())
}
