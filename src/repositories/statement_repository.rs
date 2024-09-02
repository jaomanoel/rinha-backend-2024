use chrono::Local;

use crate::{
    dto::responses::{balance_response::BalanceResponse, statement_response::StatementResponse},
    model::account::Account,
    persistence::database::Database,
    repositories::transaction_repository::TransactionRepository,
};

pub struct StatementRepository;

impl StatementRepository {
    pub fn get_statement_by_id(
        &self,
        id: u32,
        account: Account,
        db: &mut Database,
    ) -> StatementResponse {
        let transaction_repository = TransactionRepository;

        StatementResponse {
            saldo: BalanceResponse {
                total: account.saldo,
                data_extrato: Local::now(),
                limite: account.limite,
            },
            ultimas_transacoes: transaction_repository.get_transactions_by_account_id(id, db),
        }
    }
}
