use chrono::{DateTime, Local};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BalanceResponse {
    pub total: i32,
    pub data_extrato: DateTime<Local>,
    pub limite: i32,
}
