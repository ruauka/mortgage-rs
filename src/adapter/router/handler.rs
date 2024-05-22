use crate::adapter::cache::{insert, SharedState};
use crate::entities::mortgage::Mortgage;
use crate::entities::schema::{Request, Response};
use crate::errors::AppError::EmptyCache;
use crate::errors::{AppError, Result};
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
    loan.rate_calc()?;
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
