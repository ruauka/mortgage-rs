use crate::entities::mortgage::{Mortgage, Program};
use serde::{Deserialize, Serialize};

/// Запрос.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Request {
    pub object_cost: f64,
    pub initial_payment: f64,
    pub months: u8,
    pub program: Program,
}

/// Ответ кэша.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Response {
    pub id: u32,
    pub loan: Mortgage,
}

impl Response {
    /// Конструктор.
    pub fn new(id: u32, loan: Mortgage) -> Self {
        Self { id, loan }
    }
}
