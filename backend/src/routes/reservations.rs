use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

use crate::HOLD_DURATION_SECS;

#[derive(Deserialize)]
pub struct ReserveRequest {
    user_id: Uuid,
    seat_id: Option<Uuid>,
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

    let seat_id: Option<Uuid> = if let Some(req_seat_id) = body.seat_id {
        sqlx::query_scalar(
            "SELECT id FROM seats
             WHERE id = $1 AND event_id = $2 AND status = 'available'
             FOR UPDATE",
        )
        .bind(req_seat_id)
        .bind(event_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        sqlx::query_scalar(
            "SELECT id FROM seats
             WHERE event_id = $1 AND status = 'available'
             LIMIT 1
             FOR UPDATE SKIP LOCKED",
        )
        .bind(event_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    match seat_id {
        Some(seat_id) => {
            sqlx::query(
                "UPDATE seats
                 SET status = 'held',
                     held_until = now() + $1,
                     held_by_user_id = $2
                 WHERE id = $3",
            )
            .bind(Duration::from_secs(HOLD_DURATION_SECS))
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

#[derive(Serialize, sqlx::FromRow)]
pub struct EventListResponse {
    id: Uuid,
    name: String,
    date: String,
    venue: String,
    total_seats: i32,
    available_seats: i64,
    held_seats: i64,
    sold_seats: i64,
    waitlist_count: i64,
}

pub async fn get_events(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, StatusCode> {
    let events = sqlx::query_as::<_, EventListResponse>(
        r#"
        SELECT
            e.id,
            e.name,
            TO_CHAR(e.date, 'YYYY-MM-DD') AS date,
            e.venue,
            e.total_seats,
            COUNT(CASE WHEN s.status = 'available' THEN 1 END) AS available_seats,
            COUNT(CASE WHEN s.status = 'held' THEN 1 END) AS held_seats,
            COUNT(CASE WHEN s.status = 'sold' THEN 1 END) AS sold_seats,
            (SELECT COUNT(*) FROM waitlist w WHERE w.event_id = e.id) AS waitlist_count
        FROM events e
        LEFT JOIN seats s ON s.event_id = e.id
        GROUP BY e.id, e.name, e.date, e.venue, e.total_seats
        ORDER BY e.date ASC
        "#,
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch events: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(events))
}

#[derive(Serialize, sqlx::FromRow)]
pub struct SeatResponse {
    id: Uuid,
    label: Option<String>,
    status: String,
    held_until_epoch: Option<f64>,
    held_by_user_id: Option<Uuid>,
    reservation_id: Option<Uuid>,
}

pub async fn get_event_seats(
    State(pool): State<PgPool>,
    Path(event_id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    let seats = sqlx::query_as::<_, SeatResponse>(
        r#"
        SELECT
            s.id,
            s.label,
            s.status,
            EXTRACT(EPOCH FROM s.held_until)::double precision AS held_until_epoch,
            s.held_by_user_id,
            r.id AS reservation_id
        FROM seats s
        LEFT JOIN reservations r ON r.seat_id = s.id
        WHERE s.event_id = $1
        ORDER BY s.label::integer
        "#,
    )
    .bind(event_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch seats for event {}: {:?}", event_id, e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(seats))
}
