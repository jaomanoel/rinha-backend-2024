use std::env;

pub fn url_postgres() -> String {
    env::var("DATABASE_URL").unwrap().to_string()
}
