use axum::{Router, routing::post};
use sqlx::PgPool;

mod reservations;

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route(
            "/events/{event_id}/reservations",
            post(reservations::reserve),
        )
        .route(
            "/reservations/{reservation_id}/payment",
            post(reservations::pay),
        )
        .with_state(pool)
}
