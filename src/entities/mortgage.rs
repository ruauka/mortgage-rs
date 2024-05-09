use crate::entities::schema::Request;
use crate::errors::AppError;
use crate::errors::AppError::{LoanProgramEmpty, LoanProgramMoreThanOne, MinInitialPayment};
use chrono::prelude::*;
use chrono::Months;
use serde::{Deserialize, Serialize};
use std::ops::BitAnd;

// Минимальная процент первоначального взноса.
const MIN_INITIAL_PAYMENT_PERCENT: f64 = 20_f64;
// Годовая процентаня ставка зарплатника.
const SALARY: f64 = 8_f64;
// Годовая процентаня ставка военного.
const MILITARY: f64 = 9_f64;
// Годовая базовая процентаня ставка.
const BASE: f64 = 10_f64;

// Структура ипотечной программы.
#[derive(Debug, Serialize)]
pub struct Mortgage {
    pub params: Params,
    pub program: Program,
    pub aggregates: Aggregates,
}

/// Параметры кредита.
#[derive(Debug, Default, Serialize)]
pub struct Params {
    object_cost: f64,
    initial_payment: f64,
    months: u8,
}

/// Ипотечная программа.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Program {
    #[serde(skip_serializing_if = "Option::is_none")]
    base: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    military: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    salary: Option<bool>,
}

/// Расчитываемые агрегаты.
#[derive(Debug, Default, Serialize)]
pub struct Aggregates {
    rate: f64,
    loan_sum: f64,
    monthly_payment: f64,
    overpayment: f64,
    last_payment_date: String,
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

        if counter > 1 {
            return Err(LoanProgramMoreThanOne);
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
    pub fn rate_calc(&mut self) -> Result<(), AppError> {
        if self.program.salary.unwrap() {
            self.aggregates.rate = SALARY;
            Ok(())
        } else if self.program.military.unwrap() {
            self.aggregates.rate = MILITARY;
            Ok(())
        } else if self.program.base.unwrap() {
            self.aggregates.rate = BASE;
            Ok(())
        } else {
            Err(LoanProgramEmpty)
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

    /// Расчет перелпты за весь срок кредита.
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
