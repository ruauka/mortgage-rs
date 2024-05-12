use axum::{extract::Request, middleware::Next, response::Response};
use tracing::{error, info};

/// Обертка замера времени и статуса.
pub async fn middleware(request: Request, next: Next) -> Response {
    // замер времени
    let start = std::time::Instant::now();
    // вызов хендлера
    let response = next.run(request).await;
    // статус ответа хендлера
    let status = response.status();
    // замер времени
    let end = start.elapsed().as_micros();
    // логирование ответа хендлера
    if response.status().is_success() {
        info!("status_code: {}, duration: {} μs", status, end);
    } else {
        error!("status_code: {}, duration: {} μs", status, end)
    }
    // ответ декоратора
    response
}
