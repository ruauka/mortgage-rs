use crate::adapter::cache::AppState;
use crate::adapter::router::handler::{cache, mortgage};
use crate::adapter::router::middleware::middleware;
use axum::{
    middleware::{self as mw},
    routing::{get, post},
    Router,
};
use std::sync::{Arc, RwLock};

mod handler;
mod middleware;

/// Создание роутера и регистрация хендлеров.
pub async fn router(state: Arc<RwLock<AppState>>) -> Router {
    Router::new()
        .route("/execute", post(mortgage))
        .route("/cache", get(cache))
        // кастомный middleware
        .layer(mw::from_fn(middleware))
        // // axum-логер
        // .layer((
        //     TraceLayer::new_for_http()
        //         .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        //         .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        //     // Graceful shutdown
        //     TimeoutLayer::new(Duration::from_secs(5)),
        // ))
        .with_state(Arc::clone(&state))
}
