use postgres::Error;

use crate::{model::transaction::Transaction, persistence::database::Database};

pub struct TransactionRepository;

#[derive(Debug)]
pub enum TransactionRepositoryError {
    ErrorRoolback,
    ErrorSaveTransaction,
    ErrorDbTransaction,
    ErrorConnectDatabase,
}

impl TransactionRepository {
    pub fn get_transactions_by_account_id(&self, id: u32, db: &mut Database) -> Vec<Transaction> {
        let row_transactions = db.connection.query(
            r#"
                SELECT amount, transaction_type, description, created
                FROM transactions
                WHERE account_id = $1
                ORDER BY created DESC
                LIMIT 10;
            "#,
            &[&(id as i32)],
        );

        let mut transactions: Vec<Transaction> = Vec::new();

        if let Ok(vec_transactions) = row_transactions {
            vec_transactions.iter().for_each(|row| {
                transactions.push(Transaction {
                    valor: row.get("amount"),
                    tipo: row.get("transaction_type"),
                    descricao: row.get("description"),
                    realizada_em: row.get("created"),
                })
            });
        }

        transactions
    }

    pub fn create_transaction(
        &self,
        id: u32,
        value: i32,
        balance: i32,
        transaction_type: String,
        descrition: String,
        db: &mut Database,
    ) -> Result<(), TransactionRepositoryError> {
        match db.connection.transaction() {
            Ok(mut db_transaction) => {
                let db_transaction_save = db_transaction.execute(r#"
                    INSERT INTO transactions (account_id, amount, transaction_type, description) VALUES ($1, $2, $3, $4);
                "#, &[&(id as i32), &value, &transaction_type, &descrition]);

                let amount: i32 = match transaction_type.as_ref() {
                    "c" => balance + value,
                    "d" => balance - value,
                    _ => 0,
                };

                let db_clientes_update: Result<u64, Error> = db_transaction.execute(
                    r#"UPDATE accounts SET balance = $1 WHERE accounts.id = $2;"#,
                    &[&amount, &(id as i32)],
                );

                match (db_transaction_save, db_clientes_update) {
                    (Ok(_), Ok(_)) => db_transaction.commit().map_or_else(
                        |_| Err(TransactionRepositoryError::ErrorSaveTransaction),
                        |_| Ok(()),
                    ),
                    _ => db_transaction.rollback().map_or_else(
                        |_| Err(TransactionRepositoryError::ErrorRoolback),
                        |_| Err(TransactionRepositoryError::ErrorSaveTransaction),
                    ),
                }
            }
            _ => Err(TransactionRepositoryError::ErrorDbTransaction),
        }
    }
}
