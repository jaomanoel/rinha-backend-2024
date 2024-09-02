use crate::{model::account::Account, persistence::database::Database};

pub struct AccountRepository;

pub enum AccountRepositoryError {
    AccountNotFound,
}

impl AccountRepository {
    pub fn get_accout_by_id(
        &self,
        id: u32,
        db: &mut Database,
    ) -> Result<Account, AccountRepositoryError> {
        let row = db.connection.query_one(
            r#"
                SELECT limit_amount, balance 
                FROM accounts
                WHERE accounts.id = $1
                FOR UPDATE
            "#,
            &[&(id as i32)],
        );

        match row {
            Ok(account) => Ok(Account {
                limite: account.get("limit_amount"),
                saldo: account.get("balance"),
            }),
            _ => Err(AccountRepositoryError::AccountNotFound),
        }
    }
}
