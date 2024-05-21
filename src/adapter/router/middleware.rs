use axum::http::StatusCode;
use axum::{extract::Request, middleware::Next, response::Response};
use std::time::Instant;
use tracing::{error, info};

/// Middleware.
pub async fn middleware(request: Request, next: Next) -> Response {
    // эндпоит
    let path: &String = &request.uri().path().to_string();
    // старт времени
    let start: Instant = Instant::now();
    // вызов хендлера
    let response: Response = next.run(request).await;
    // статус ответа хендлера
    let status: StatusCode = response.status();
    // стоп времени
    let end: u128 = start.elapsed().as_micros();
    // логирование ответа хендлера
    if response.status().is_success() {
        info!(
            "path={}, status=Success, status_code={}, duration={} μs",
            path, status, end
        );
    } else {
        error!(
            "path={}, status=Error, status_code={}, duration={} μs",
            path, status, end
        )
    }
    // ответ декоратора
    response
}
