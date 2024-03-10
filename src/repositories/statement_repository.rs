use chrono::Local;

use crate::{
    dto::responses::{balance_response::BalanceResponse, statement_response::StatementResponse},
    model::account::Account,
    repositories::transaction_repository::TransactionRepository,
};

pub struct StatementRepository;

pub enum StatementRepositoryError {
    ConnectDatabase,
    Transaction,
}

impl StatementRepository {
    pub fn get_statement_by_id(
        &self,
        id: i32,
        account: Account,
    ) -> Result<StatementResponse, StatementRepositoryError> {
        let transaction_repository = TransactionRepository;

        match transaction_repository.get_transactions_by_account_id(id) {
            Ok(transactions) => Ok(StatementResponse {
                saldo: BalanceResponse {
                    total: account.saldo,
                    data_extrato: Local::now(),
                    limite: account.limite,
                },
                ultimas_transacoes: transactions,
            }),
            _ => Err(StatementRepositoryError::Transaction),
        }
    }
}
