use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use thiserror::Error;

/// Переназначение Result для ответов хендлеров.
pub type Result<T, E = AppError> = core::result::Result<T, E>;

/// Ошибки сервиса.
#[derive(Debug, PartialEq, Error)]
pub enum AppError {
    // не выбрана программа кредитования
    #[error("choose credit program")]
    LoanProgramEmpty,
    // выбрано несколько программ кредитования
    #[error("choose only 1 credit program")]
    LoanProgramMoreThanOne,
    // первоначальный взнос ниже допустимого значения
    #[error("the initial payment should be more")]
    MinInitialPayment,
    // пустой кэш
    #[error("empty cache")]
    EmptyCache,
}

/// Имплементация для Axum Response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            AppError::LoanProgramEmpty
            | AppError::LoanProgramMoreThanOne
            | AppError::MinInitialPayment
            | AppError::EmptyCache => (StatusCode::BAD_REQUEST, self.to_string()),
        };
        let body = Json(json!({
            "error": err_msg,
        }));
        (status, body).into_response()
    }
}
