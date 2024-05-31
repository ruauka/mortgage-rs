use crate::adapter::cache::{insert, SharedState};
use crate::errors::AppError::EmptyCache;
use crate::errors::{AppError, Result};
use crate::logic::mortgage::Mortgage;
use crate::schema::{Request, Response};
use axum::extract::State;
use axum::Json;
use std::collections::HashMap;

/// Эндпоинт расчета ипотеки.
pub async fn mortgage(
    State(state): State<SharedState>,
    Json(req): Json<Request>,
) -> Result<Json<Response>, AppError> {
    // объект кредита с нужными полями
    let mut loan: Mortgage = Mortgage::new(req);
    // проверка на наличие больше 1 программы в запросе
    loan.loan_program_check()?;
    // проверка минимальной суммы первоначального взноса
    loan.min_initial_payment_check()?;
    // расчет суммы кредита
    loan.loan_sum_calc();
    // Определение процентной ставки
    loan.rate_calc();
    // расчет ежемесячного платежа
    loan.monthly_payment_calc();
    // расчет переплаты
    loan.overpayment_calc();
    // расчет даты последнего платежа
    loan.last_payment_date_calc();
    // запись расчета в кэш
    let id: u32 = insert(state, loan.clone()).await;
    // формирование ответа
    let resp: Response = Response::new(id, loan);
    // ответ 200
    Ok(Json(resp))
}

/// Получение из кэша всех расчитанных ипотек.
pub async fn cache(State(state): State<SharedState>) -> Result<Json<Vec<Response>>, AppError> {
    // получение кэша
    let cache: &HashMap<u32, Mortgage> = &state.read().unwrap().cache;
    // проверка на пустой кэш
    if cache.is_empty() {
        return Err(EmptyCache);
    }
    // формирование ответа
    let mut resp: Vec<Response> = Vec::with_capacity(cache.len());
    let mut r: Response = Response::default();
    // перекладка из кэша
    for (k, v) in cache.iter() {
        r.id = *k;
        r.loan = v.clone();
        resp.push(r.clone())
    }
    // ответ 200
    Ok(Json(resp))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::cache::AppState;
    use crate::adapter::router::router;
    use crate::logic::mortgage::{Aggregates, Params, Program};
    use crate::schema::Request as Req;
    use axum::{
        body::{Body, Bytes},
        http::{self, Request, StatusCode},
    };
    use chrono::{Months, Utc};
    use http_body_util::BodyExt;
    use serde_json::{json, Value};
    use std::sync::{Arc, RwLock};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_mortgage() {
        let state: Arc<RwLock<AppState>> = SharedState::default();
        let router = router(state).await;
        let req = Req {
            object_cost: 100.0,
            initial_payment: 30.0,
            months: 12,
            program: Program {
                base: Some(true),
                military: None,
                salary: None,
            },
        };

        let resp = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/execute")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::to_vec(&json!(req)).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);

        let expected: Json<Response> = Json(Response {
            id: 0,
            loan: Mortgage {
                params: Params {
                    object_cost: 100.0,
                    initial_payment: 30.0,
                    months: 12,
                },
                program: Program {
                    base: Some(true),
                    military: None,
                    salary: None,
                },
                aggregates: Aggregates {
                    rate: 10.0,
                    loan_sum: 70.0,
                    monthly_payment: 7.0,
                    overpayment: 14.0,
                    last_payment_date: Utc::now()
                        .checked_add_months(Months::new(12_u32))
                        .unwrap()
                        .format("%Y-%m-%d")
                        .to_string(),
                },
            },
        });

        let body: Bytes = resp.into_body().collect().await.unwrap().to_bytes();
        let actual: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(actual, json!(*expected));
    }

    #[tokio::test]
    async fn test_cache() {
        let state: Arc<RwLock<AppState>> = SharedState::default();
        let router = router(state).await;

        let resp = router
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/cache")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

        let body: Bytes = resp.into_body().collect().await.unwrap().to_bytes();
        let actual: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(actual, json!({"error": EmptyCache.to_string()}));
    }
}
