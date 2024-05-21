use axum::{extract::Request, middleware::Next, response::Response};
use tracing::{error, info};

/// Middleware.
pub async fn middleware(request: Request, next: Next) -> Response {
    // эндпоит
    let path = &request.uri().path().to_string();
    // старт времени
    let start = std::time::Instant::now();
    // вызов хендлера
    let response = next.run(request).await;
    // статус ответа хендлера
    let status = response.status();
    // стоп времени
    let end = start.elapsed().as_micros();
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
