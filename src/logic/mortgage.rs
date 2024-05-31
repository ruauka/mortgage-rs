use crate::errors::AppError;
use crate::errors::AppError::{LoanProgramEmpty, LoanProgramMoreThanOne, MinInitialPayment};
use crate::schema::Request;
use chrono::prelude::*;
use chrono::Months;
use serde::{Deserialize, Serialize};

// Минимальная процент первоначального взноса.
const MIN_INITIAL_PAYMENT_PERCENT: f64 = 20_f64;
// Годовая процентаня ставка зарплатника.
const SALARY: f64 = 8_f64;
// Годовая процентаня ставка военного.
const MILITARY: f64 = 9_f64;
// Годовая базовая процентаня ставка.
const BASE: f64 = 10_f64;

// Структура ипотечной программы.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Mortgage {
    pub params: Params,
    pub program: Program,
    pub aggregates: Aggregates,
}

/// Параметры кредита.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Params {
    pub object_cost: f64,
    pub initial_payment: f64,
    pub months: u8,
}

/// Ипотечная программа.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Program {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub military: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salary: Option<bool>,
}

/// Расчитываемые агрегаты.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Aggregates {
    pub rate: f64,
    pub loan_sum: f64,
    pub monthly_payment: f64,
    pub overpayment: f64,
    pub last_payment_date: String,
}

impl Mortgage {
    /// Конструктор.
    pub fn new(req: Request) -> Self {
        Self {
            params: Params {
                object_cost: req.object_cost,
                initial_payment: req.initial_payment,
                months: req.months,
            },
            program: Program {
                base: req.program.base,
                military: req.program.military,
                salary: req.program.salary,
            },
            aggregates: Aggregates::default(),
        }
    }

    /// Проверка на наличие больше 1 программы в запросе.
    pub fn loan_program_check(&mut self) -> Result<(), AppError> {
        let mut counter: i8 = i8::default();

        let check_vec: Vec<bool> = vec![
            self.program.salary.unwrap_or_default(),
            self.program.military.unwrap_or_default(),
            self.program.base.unwrap_or_default(),
        ];

        for program in check_vec {
            if program {
                counter += 1;
            }
        }
        // проверка, что выбрано не больше 1 программы кредитования
        if counter > 1 {
            return Err(LoanProgramMoreThanOne);
        }
        // проверка, что выбрана хотя бы 1 программа кредитования
        if counter == 0 {
            return Err(LoanProgramEmpty);
        }

        Ok(())
    }

    /// Проверка минимальной суммы первоначального взноса.
    pub fn min_initial_payment_check(&self) -> Result<(), AppError> {
        if self.params.initial_payment
            < self.params.object_cost * MIN_INITIAL_PAYMENT_PERCENT / 100_f64
        {
            return Err(MinInitialPayment);
        }

        Ok(())
    }

    /// Расчет суммы кредита.
    pub fn loan_sum_calc(&mut self) {
        self.aggregates.loan_sum = self.params.object_cost - self.params.initial_payment
    }

    /// Определение процентной ставки.
    pub fn rate_calc(&mut self) {
        if self.program.salary.unwrap_or_default() {
            self.aggregates.rate = SALARY;
        } else if self.program.military.unwrap_or_default() {
            self.aggregates.rate = MILITARY;
        } else if self.program.base.unwrap_or_default() {
            self.aggregates.rate = BASE;
        }
    }

    /// Pасчет ежемесячного аннуитетного платежа
    pub fn monthly_payment_calc(&mut self) {
        let monthly_rate: f64 = self.aggregates.rate / 100_f64 / 12_f64;
        let pow_monthly_rate: f64 = (1_f64 + monthly_rate).powf(self.params.months as f64);

        self.aggregates.monthly_payment =
            (self.aggregates.loan_sum * monthly_rate * pow_monthly_rate
                / (pow_monthly_rate - 1_f64))
                // округление вверх
                .ceil()
    }

    /// Расчет переплаты за весь срок кредита.
    pub fn overpayment_calc(&mut self) {
        self.aggregates.overpayment =
            self.aggregates.monthly_payment * self.params.months as f64 - self.aggregates.loan_sum
    }

    /// Расчет даты последнего платежа.
    pub fn last_payment_date_calc(&mut self) {
        self.aggregates.last_payment_date = Utc::now()
            .checked_add_months(Months::new(self.params.months as u32))
            .unwrap()
            .format("%Y-%m-%d")
            .to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loan_program_check() {
        // Ok. Выбрана одна прорамма кредитования
        let mut loan: Mortgage = Mortgage::default();
        loan.program.salary = Some(true);
        let actual: bool = loan.loan_program_check().is_ok();
        assert_eq!(actual, true);
        // Err. Выбрано 2 прораммы кредитования
        let mut loan: Mortgage = Mortgage::default();
        loan.program.salary = Some(true);
        loan.program.military = Some(true);
        let actual: AppError = loan.loan_program_check().err().unwrap();
        assert_eq!(actual, LoanProgramMoreThanOne);
        // Err. Не выбрано ни одной прораммы кредитования
        let mut loan: Mortgage = Mortgage::default();
        let actual: AppError = loan.loan_program_check().err().unwrap();
        assert_eq!(actual, LoanProgramEmpty)
    }

    #[test]
    fn test_min_initial_payment_check() {
        let mut loan: Mortgage = Mortgage::default();
        loan.params.object_cost = 100_f64;
        // Ok. Допустимый минимальный платеж
        loan.params.initial_payment = 25_f64;
        let actual: bool = loan.min_initial_payment_check().is_ok();
        assert_eq!(actual, true);
        // Err. Первоначальный взнос меньше минимально допустимого.
        loan.params.initial_payment = 10_f64;
        let actual: AppError = loan.min_initial_payment_check().err().unwrap();
        assert_eq!(actual, MinInitialPayment)
    }

    #[test]
    fn test_loan_sum_calc() {
        let mut loan: Mortgage = Mortgage::default();
        loan.params.object_cost = 100_f64;
        loan.params.initial_payment = 25_f64;
        loan.loan_sum_calc();
        assert_eq!(loan.aggregates.loan_sum, 75_f64)
    }

    #[test]
    fn test_rate_calc() {
        let mut loan: Mortgage = Mortgage::default();
        loan.program.salary = Some(true);
        loan.rate_calc();
        assert_eq!(loan.aggregates.rate, SALARY);

        let mut loan: Mortgage = Mortgage::default();
        loan.program.military = Some(true);
        loan.rate_calc();
        assert_eq!(loan.aggregates.rate, MILITARY);

        let mut loan: Mortgage = Mortgage::default();
        loan.program.base = Some(true);
        loan.rate_calc();
        assert_eq!(loan.aggregates.rate, BASE)
    }

    #[test]
    fn test_monthly_payment_calc() {
        let mut loan: Mortgage = Mortgage::default();
        loan.params.months = 240;
        loan.aggregates.loan_sum = 999_999_f64;

        loan.aggregates.rate = SALARY;
        loan.monthly_payment_calc();
        assert_eq!(loan.aggregates.monthly_payment, 8365_f64);

        loan.aggregates.rate = MILITARY;
        loan.monthly_payment_calc();
        assert_eq!(loan.aggregates.monthly_payment, 8998_f64);

        loan.aggregates.rate = BASE;
        loan.monthly_payment_calc();
        assert_eq!(loan.aggregates.monthly_payment, 9651_f64);
    }

    #[test]
    fn test_overpayment_calc() {
        let mut loan: Mortgage = Mortgage::default();
        loan.aggregates.monthly_payment = 100_f64;
        loan.params.months = 60;
        loan.aggregates.loan_sum = 1000_f64;
        loan.overpayment_calc();
        assert_eq!(loan.aggregates.overpayment, 5000_f64)
    }

    #[test]
    fn test_last_payment_date_calc() {
        let mut loan: Mortgage = Mortgage::default();
        loan.params.months = 240;
        loan.last_payment_date_calc();
        let expected = Utc::now()
            .checked_add_months(Months::new(loan.params.months as u32))
            .unwrap()
            .format("%Y-%m-%d")
            .to_string();
        assert_eq!(loan.aggregates.last_payment_date, expected)
    }
}
