use postgres::{Client, Error, NoTls};

use crate::{model::transaction::Transaction, persistence::database_url::url_postgres};

pub struct TransactionRepository;

#[derive(Debug)]
pub enum TransactionRepositoryError {
    ErrorRoolback,
    ErrorSaveTransaction,
    ErrorDbTransaction,
    ErrorConnectDatabase,
}

impl TransactionRepository {
    pub fn get_transactions_by_account_id(
        &self,
        id: i32,
    ) -> Result<Vec<Transaction>, TransactionRepositoryError> {
        let client = Client::connect(&url_postgres(), NoTls);

        match client {
            Ok(mut db) => {
                let row_transactions = db.query(
                    r#"
                        SELECT valor, tipo, descricao, realizada_em
                        FROM transacoes
                        WHERE cliente_id = $1
                        ORDER BY realizada_em DESC
                        LIMIT 10;
                    "#,
                    &[&id],
                );

                let mut transactions: Vec<Transaction> = Vec::new();

                if let Ok(vec_transactions) = row_transactions {
                    vec_transactions.iter().for_each(|row| {
                        transactions.push(Transaction {
                            valor: row.get("valor"),
                            tipo: row.get("tipo"),
                            descricao: row.get("descricao"),
                            realizada_em: row.get("realizada_em"),
                        })
                    });
                }

                Ok(transactions)
            }
            _ => Err(TransactionRepositoryError::ErrorConnectDatabase),
        }
    }

    pub fn create_transaction(
        &self,
        id: i32,
        value: i32,
        limit: i32,
        balance: i32,
        transaction_type: String,
        descrition: String,
    ) -> Result<(), TransactionRepositoryError> {
        let client = Client::connect(&url_postgres(), NoTls);

        match client {
            Ok(mut db) => {
                let db_transaction_result = db.transaction();

                match db_transaction_result {
                    Ok(mut db_transaction) => {
                        let db_transaction_save = db_transaction.execute(
                            r#"INSERT INTO transacoes (cliente_id, valor, tipo, descricao) VALUES ($1, $2, $3, $4);
                        "#, &[&id, &value, &transaction_type, &descrition]);

                        let db_saldo_update = db_transaction.execute(
                            r#"UPDATE saldos SET valor = $1 WHERE cliente_id = $2;
                        "#,
                            &[&(balance - value), &id],
                        );

                        let amount: i32 = match transaction_type.as_ref() {
                            "c" => limit + value,
                            "d" => limit - value,
                            _ => 0,
                        };

                        let db_clientes_update: Result<u64, Error> = db_transaction.execute(
                            r#"UPDATE clientes SET limite = $1 WHERE id = $2;"#,
                            &[&amount, &id],
                        );

                        match (db_transaction_save, db_saldo_update, db_clientes_update) {
                            (Ok(_), Ok(_), Ok(_)) => db_transaction.commit().map_or_else(
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
            _ => Err(TransactionRepositoryError::ErrorConnectDatabase),
        }
    }
}
