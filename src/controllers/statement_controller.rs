use std::sync::Arc;

use serde_json::{json, Value};

use crate::{
    http_status::http_type::HttpType,
    id_is_number,
    persistence::database::Database,
    queue::Queue,
    repositories::{
        account_repository::{AccountRepository, AccountRepositoryError},
        statement_repository::StatementRepository,
    },
};

pub fn controller(url: &str, queue_db: Arc<Queue<Database>>) -> (HttpType, Value) {
    let mut response_status: HttpType = HttpType::BadRequest;
    let mut response_json: Value = json!({"message": "Bad Request"});

    if url.contains("/clientes/") && url.contains("/extrato") {
        if let Some(id) = id_is_number(url) {
            let mut db = queue_db.pop_back();
            let account_repository = AccountRepository;

            match account_repository.get_accout_by_id(id, &mut db) {
                Ok(account) => {
                    let statement_repository = StatementRepository;

                    response_status = HttpType::Ok;
                    response_json =
                        json!(statement_repository.get_statement_by_id(id, account, &mut db));
                }
                Err(AccountRepositoryError::AccountNotFound) => {
                    response_status = HttpType::NotFound;
                    response_json = json!({"message": "Id not Found"});
                }
            }

            queue_db.push_front(db);
        }
    }

    (response_status, response_json)
}
