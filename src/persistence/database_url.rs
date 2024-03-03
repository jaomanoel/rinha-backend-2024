use std::env;

pub fn url_postgres() -> String {
    format!(
        "host={} user={} password={} dbname={}",
        env::var("POSTGRES_HOST").unwrap(),
        env::var("POSTGRES_USER").unwrap(),
        env::var("POSTGRES_PASSWORD").unwrap(),
        env::var("POSTGRES_DB").unwrap()
    )
}
