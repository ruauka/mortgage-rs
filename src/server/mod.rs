use crate::adapter::cache::{AppState, SharedState};
use crate::adapter::router::router;
use crate::config::Cli;
use axum::Router;
use clap::Parser;
use std::sync::{Arc, RwLock};
use tokio::signal;
use tracing::info;

/// Основная функция. Инициализация и запуск сервиса.
pub async fn execute() {
    // cli-конфиг
    let cfg: Cli = Cli::parse();
    // включение трейсинга
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
    // создание 'state' объекта
    let state: Arc<RwLock<AppState>> = SharedState::default();
    // хост и порт
    let address: String = format!("{}:{}", cfg.host, cfg.port);
    // создание роутера и регистрация хендлеров
    let router: Router = router(state).await;
    // tcp-движок
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    info!(
        "🚀 Server started successfully. Listening on {}...",
        listener.local_addr().unwrap()
    );
    // запуск сервиса с graceful shutdown
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// Graceful shutdown.
async fn shutdown_signal() {
    // сигнал "ctrl_c"
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    // сигнал terminate
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };
    // отслеживание всех сигналов завершения
    tokio::select! {
        _ = ctrl_c => { info!("Shutting down server...") },
        _ = terminate => { info!("Shutting down server...") },
    }
}
