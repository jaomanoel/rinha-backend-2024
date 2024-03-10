use postgres::{Client, NoTls};

use super::database_url;

pub struct Database {
    pub connection: Client,
}

pub enum DatabaseError {
    DatabaseConnection,
}

impl Database {
    pub fn new() -> Result<Database, DatabaseError> {
        let client = Client::connect(&database_url::url_postgres(), NoTls);

        match client {
            Ok(client) => Ok(Database { connection: client }),
            _ => Err(DatabaseError::DatabaseConnection),
        }
    }
}
