use crate::entities::mortgage::{Aggregates, Mortgage, Params, Program};
use serde::{Deserialize, Serialize};

/// Запрос.
#[derive(Debug, Default, Deserialize)]
pub struct Request {
    pub object_cost: f64,
    pub initial_payment: f64,
    pub months: u8,
    pub program: Program,
}

/// Ответ кэша.
#[derive(Debug, Default, Serialize)]
pub struct Response {
    pub id: u32,
    pub params: Params,
    pub program: Program,
    pub aggregates: Aggregates,
}
