use chrono::Local;

use crate::dto::responses::{
    balance_response::BalanceResponse, statement_response::StatementResponse,
};

use super::{
    account_repository::{AccountRepository, AccountRepositoryError},
    transaction_repository::TransactionRepository,
};

pub struct StatementRepository;

pub enum StatementRepositoryError {
    ConnectDatabase,
    AccountNotFound,
    Transaction,
}

impl StatementRepository {
    pub fn get_statement_by_id(
        &self,
        id: i32,
    ) -> Result<StatementResponse, StatementRepositoryError> {
        let account_repository = AccountRepository;

        match account_repository.get_accout_by_id(id) {
            Ok(account) => {
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
            Err(AccountRepositoryError::AccountNotFound) => {
                Err(StatementRepositoryError::AccountNotFound)
            }
            _ => Err(StatementRepositoryError::ConnectDatabase),
        }
    }
}
