use axum::{Router, routing::post};
use sqlx::PgPool;

mod reservations;

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route(
            "/events/{event_id}/reservations",
            post(reservations::reserve),
        )
        .with_state(pool)
}
