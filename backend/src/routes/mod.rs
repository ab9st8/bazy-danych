use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

mod reservations;

pub fn router(pool: PgPool) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/events", get(reservations::get_events))
        .route(
            "/events/{event_id}/seats",
            get(reservations::get_event_seats),
        )
        .route(
            "/events/{event_id}/reservations",
            post(reservations::reserve),
        )
        .route(
            "/reservations/{reservation_id}/payment",
            post(reservations::pay),
        )
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(pool)
}
