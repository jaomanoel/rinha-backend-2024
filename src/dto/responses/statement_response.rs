use serde::Serialize;

use crate::model::transaction::Transaction;

use super::balance_response::BalanceResponse;

#[derive(Debug, Serialize)]
pub struct StatementResponse {
    pub saldo: BalanceResponse,
    pub ultimas_transacoes: Vec<Transaction>,
}
