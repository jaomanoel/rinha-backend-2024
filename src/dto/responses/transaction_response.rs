use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionResponse {
    pub limite: i32,
    pub saldo: i32,
}
