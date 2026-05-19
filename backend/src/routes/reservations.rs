use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
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

#[tracing::instrument(skip(pool, body), fields(user_id = %body.user_id))]
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
