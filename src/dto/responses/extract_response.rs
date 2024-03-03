use serde::Serialize;

use super::{
    balance_response::BalanceResponse, transaction_extract_response::TransactionExtractResponse,
};

#[derive(Debug, Serialize)]
pub struct ExtractResponse {
    pub saldo: BalanceResponse,
    pub ultimas_transacoes: Vec<TransactionExtractResponse>,
}
