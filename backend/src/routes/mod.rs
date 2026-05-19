use axum::{Router, routing::post};
use sqlx::PgPool;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

mod reservations;

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route(
            "/events/{event_id}/reservations",
            post(reservations::reserve),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(pool)
}
