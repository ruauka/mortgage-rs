use crate::entities::mortgage::Mortgage;
use crate::entities::schema::{Request, Response};
use crate::errors::AppError;
use crate::errors::AppError::MinInitialPayment;
use axum::Json;

/// Эндпоинт расчета ипотеки.
pub async fn mortgage(Json(req): Json<Request>) -> Result<Json<Mortgage>, AppError> {
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

    // println!("{:?}", loan);

    Ok(Json(loan))
}
