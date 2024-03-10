use postgres::{Client, NoTls};

use crate::{model::account::Account, persistence::database_url::url_postgres};

pub struct AccountRepository;

pub enum AccountRepositoryError {
    AccountNotFound,
    ConnectionError,
}

impl AccountRepository {
    pub fn get_accout_by_id(&self, id: i32) -> Result<Account, AccountRepositoryError> {
        let client = Client::connect(&&url_postgres(), NoTls);

        match client {
            Ok(mut db) => {
                let row = db.query_one(
                    r#"
                    SELECT clientes.limite, saldos.valor 
                    FROM clientes
                    LEFT JOIN saldos ON clientes.id = saldos.cliente_id
                    WHERE cliente_id = $1;
                "#,
                    &[&id],
                );

                match row {
                    Ok(account) => Ok(Account {
                        limite: account.get("limite"),
                        saldo: account.get("valor"),
                    }),
                    _ => Err(AccountRepositoryError::AccountNotFound),
                }
            }
            _ => Err(AccountRepositoryError::ConnectionError),
        }
    }
}
