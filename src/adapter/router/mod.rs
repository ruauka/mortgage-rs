use std::time::Duration;

use crate::adapter::router::handler::mortgage;
use axum::routing::post;
use axum::Router;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;

mod handler;

/// Создание роутера и регистрация хендлеров.
pub async fn router() -> Router {
    Router::new().route("/execute", post(mortgage)).layer((
        TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        // Graceful shutdown
        TimeoutLayer::new(Duration::from_secs(5)),
    ))
}
