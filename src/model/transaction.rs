use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Transaction {
    pub valor: i32,
    pub tipo: String,
    pub descricao: String,
    pub realizada_em: NaiveDateTime,
}
