use std::sync::Arc;

use serde_json::{json, Value};

use crate::{
    dto::{
        requests::transaction_request::TransactionRequest,
        responses::transaction_response::TransactionResponse,
    },
    http_status::http_type::HttpType,
    id_is_number,
    model::account::Account,
    persistence::database::Database,
    queue::Queue,
    repositories::{
        account_repository::AccountRepository, transaction_repository::TransactionRepository,
    },
};

pub enum TransactionValidity {
    InvalidValue,
    InvalidType,
    InvalidDescription,
    Valid,
}

fn transaction_validity(transaction: &TransactionRequest) -> (TransactionValidity, &'static str) {
    match transaction {
        transaction if transaction.valor < 0 => (
            TransactionValidity::InvalidValue,
            "Valor deve ser maior que 0",
        ),
        transaction if transaction.tipo != "c" && transaction.tipo != "d" => (
            TransactionValidity::InvalidType,
            "Tipo da transacao deve ser c ou d",
        ),
        transaction if transaction.descricao.is_empty() || transaction.descricao.len() > 10 => (
            TransactionValidity::InvalidDescription,
            "Descricao deve ser de 1 a 10 caracteres",
        ),
        _ => (TransactionValidity::Valid, "Valid"),
    }
}

pub fn credit_service(
    id: u32,
    account: Account,
    transaction: TransactionRequest,
    transaction_repository: TransactionRepository,
    db: &mut Database,
) -> (HttpType, Value) {
    let response_status: HttpType;
    let response_json: Value;

    match transaction_repository.create_transaction(
        id,
        transaction.valor,
        account.saldo,
        transaction.tipo,
        transaction.descricao,
        db,
    ) {
        Ok(_) => {
            response_status = HttpType::Ok;
            response_json = json!(TransactionResponse {
                limite: account.limite,
                saldo: account.saldo - transaction.valor
            });
        }
        _ => {
            response_status = HttpType::InternalError;
            response_json = json!({"message": "Internal server error"});
        }
    }

    (response_status, response_json)
}

pub fn debit_service(
    id: u32,
    account: Account,
    transaction: TransactionRequest,
    transaction_repository: TransactionRepository,
    db: &mut Database,
) -> (HttpType, Value) {
    let mut response_status = HttpType::UnprocessableEntity;
    let mut response_json = json!({"message": "value is not within the limit"});

    match (account.saldo + account.limite) >= transaction.valor {
        true => match transaction_repository.create_transaction(
            id,
            transaction.valor,
            account.saldo,
            transaction.tipo,
            transaction.descricao,
            db,
        ) {
            Ok(_) => {
                response_status = HttpType::Ok;
                response_json = json!(TransactionResponse {
                    limite: account.limite,
                    saldo: account.saldo - transaction.valor
                })
            }
            _ => {
                response_status = HttpType::InternalError;
                response_json = json!({"message": "Internal server error"});
            }
        },

        false => {}
    }

    (response_status, response_json)
}

pub fn controller(url: &str, body: String, queue_db: Arc<Queue<Database>>) -> (HttpType, Value) {
    let mut response_status: HttpType = HttpType::BadRequest;
    let mut response_json: Value = json!({"message": "Bad Request"});

    if url.contains("/clientes/") && url.contains("/transacoes") {
        match id_is_number(url) {
            Some(id) => {
                let transaction_optional: Result<TransactionRequest, serde_json::Error> =
                    serde_json::from_str(&body);

                match transaction_optional {
                    Ok(transaction) => match transaction_validity(&transaction) {
                        (TransactionValidity::Valid, _) => {
                            let mut db = queue_db.pop_back();
                            let account_repository = AccountRepository;

                            match account_repository.get_accout_by_id(id, &mut db) {
                                Ok(account) => {
                                    let transaction_repository = TransactionRepository;

                                    match transaction.tipo.as_ref() {
                                        "c" => {
                                            let response_credit = credit_service(
                                                id,
                                                account,
                                                transaction,
                                                transaction_repository,
                                                &mut db,
                                            );
                                            response_status = response_credit.0;
                                            response_json = response_credit.1;
                                        }
                                        "d" => {
                                            let response_debit = debit_service(
                                                id,
                                                account,
                                                transaction,
                                                transaction_repository,
                                                &mut db,
                                            );
                                            response_status = response_debit.0;
                                            response_json = response_debit.1;
                                        }
                                        _ => {
                                            response_status = HttpType::UnprocessableEntity;
                                            response_json = json!({"message": "undefined type"})
                                        }
                                    }
                                }
                                _ => {
                                    response_status = HttpType::NotFound;
                                    response_json = json!({"message": "Id not found"});
                                }
                            }

                            queue_db.push_front(db);
                        }
                        (_, message) => response_json = json!({"message": message}),
                    },
                    _ => response_json = json!({"message": "Todos os campos sao obrigatórios"}),
                }
            }
            None => {
                response_status = HttpType::NotFound;
                response_json = json!({"message": "Id not found"});
            }
        }
    }

    (response_status, response_json)
}
