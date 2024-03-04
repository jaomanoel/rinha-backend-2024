use std::net::TcpStream;

use serde_json::json;

use crate::{
    dto::{
        requests::transaction_request::TransactionRequest,
        responses::transaction_response::TransactionResponse,
    },
    http_status::{http_response::http_response, http_type::HttpType},
    id_is_number,
    persistence::{
        account_repository::AccountRepository, transaction_repository::TransactionRepository,
    },
};

pub fn controller(url: &str, body: String, socket: TcpStream) {
    match url {
        url if url.contains("/clientes/") => match id_is_number(url) {
            Some(id) => match id.is_negative() {
                true => http_response(
                    socket,
                    HttpType::BadRequest,
                    json!({"message": "Bad Request - Invalid Id"}),
                ),
                false => match url.contains(&format!("{id}/transacoes")) {
                    true => {
                        let transaction_optional: Result<TransactionRequest, serde_json::Error> =
                            serde_json::from_str(&body);

                        match transaction_optional {
                            Ok(transaction) => match transaction {
                                trans if trans.valor < 0 => http_response(
                                    socket,
                                    HttpType::BadRequest,
                                    json!({"message": "Valor deve ser maior que 0"}),
                                ),
                                trans if trans.tipo != "c" && trans.tipo != "d" => http_response(
                                    socket,
                                    HttpType::BadRequest,
                                    json!({"message": "Tipo deve ser c ou d"}),
                                ),
                                trans
                                    if trans.descricao.is_empty() || trans.descricao.len() > 10 =>
                                {
                                    http_response(
                                        socket,
                                        HttpType::BadRequest,
                                        json!({"message": "Length 1 a 10 Description"}),
                                    )
                                }
                                _ => {
                                    let account_repository = AccountRepository;
                                    let transaction_repository = TransactionRepository;

                                    match account_repository.get_accout_by_id(id) {
                                        Ok(account) => match transaction.tipo.as_ref() {
                                            "c" => match transaction.valor < account.limite {
                                                true => match transaction_repository
                                                    .create_transaction(
                                                        id,
                                                        transaction.valor,
                                                        account.limite,
                                                        account.saldo,
                                                        transaction.tipo,
                                                        transaction.descricao,
                                                    ) {
                                                    Ok(_) => http_response(
                                                        socket,
                                                        HttpType::Ok,
                                                        json!(TransactionResponse {
                                                            limit: account.limite
                                                                - transaction.valor,
                                                            saldo: transaction.valor
                                                                + account.saldo
                                                        }),
                                                    ),
                                                    _ => http_response(
                                                        socket,
                                                        HttpType::InternalError,
                                                        json!({"message": "Internal server error"}),
                                                    ),
                                                },
                                                false => http_response(
                                                    socket,
                                                    HttpType::UnprocessableEntity,
                                                    json!({"message": "valor ultrapassa o limite"}),
                                                ),
                                            },
                                            "d" => match (account.limite - transaction.valor) > 0 {
                                                true => match transaction_repository
                                                    .create_transaction(
                                                        id,
                                                        transaction.valor,
                                                        account.limite,
                                                        account.saldo,
                                                        transaction.tipo,
                                                        transaction.descricao,
                                                    ) {
                                                    Ok(_) => http_response(
                                                        socket,
                                                        HttpType::Ok,
                                                        json!(TransactionResponse {
                                                            limit: account.limite,
                                                            saldo: transaction.valor
                                                                + account.saldo
                                                        }),
                                                    ),
                                                    _ => http_response(
                                                        socket,
                                                        HttpType::InternalError,
                                                        json!({"message": "Internal server error"}),
                                                    ),
                                                },

                                                false => http_response(
                                                    socket,
                                                    HttpType::UnprocessableEntity,
                                                    json!({"message": "value is not within the limit"}),
                                                ),
                                            },
                                            _ => http_response(
                                                socket,
                                                HttpType::UnprocessableEntity,
                                                json!({"message": "undefined type"}),
                                            ),
                                        },
                                        _ => http_response(
                                            socket,
                                            HttpType::NotFound,
                                            json!({"message": "id not found"}),
                                        ),
                                    };
                                }
                            },
                            _ => http_response(
                                socket,
                                HttpType::BadRequest,
                                json!({"message": "Todos os campos são obrigatórios."}),
                            ),
                        }
                    }
                    false => http_response(
                        socket,
                        HttpType::NotFound,
                        json!({"message": "Not Found Id"}),
                    ),
                },
            },
            None => http_response(
                socket,
                HttpType::BadRequest,
                json!({"message": "Bad Request"}),
            ),
        },
        _ => http_response(
            socket,
            HttpType::BadRequest,
            json!({"message": "Bad Request"}),
        ),
    }
}
