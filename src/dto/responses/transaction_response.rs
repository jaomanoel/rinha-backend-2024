use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionResponse {
    pub limit: i32,
    pub saldo: i32,
}
