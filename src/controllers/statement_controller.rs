use std::net::TcpStream;

use serde_json::json;

use crate::{
    http_status::{http_response::http_response, http_type::HttpType},
    id_is_number,
    persistence::statement_repository::{StatementRepository, StatementRepositoryError},
};

pub fn controller(url: &str, socket: TcpStream) {
    match url {
        url if url.contains("/clientes/") => match id_is_number(url) {
            Some(id) => match id.is_negative() {
                true => http_response(
                    socket,
                    HttpType::BadRequest,
                    json!({"message": "Id is invalid"}),
                ),
                false => match url.contains(&format!("{id}/extrato")) {
                    true => {
                        let statement_repository = StatementRepository;

                        match statement_repository.get_statement_by_id(id) {
                            Ok(statement) => http_response(socket, HttpType::Ok, json!(statement)),
                            Err(StatementRepositoryError::AccountNotFound) => http_response(
                                socket,
                                HttpType::NotFound,
                                json!({"message": "Not Found"}),
                            ),
                            _ => http_response(
                                socket,
                                HttpType::InternalError,
                                json!({"message": "Internal Server Error"}),
                            ),
                        }
                    }
                    false => http_response(
                        socket,
                        HttpType::BadRequest,
                        json!({"message": "Bad Request"}),
                    ),
                },
            },
            None => http_response(
                socket,
                HttpType::BadRequest,
                json!({"message": "Bad Request - Invalid ID"}),
            ),
        },
        _ => http_response(
            socket,
            HttpType::BadRequest,
            json!({"message": "Bad Request"}),
        ),
    }
}
