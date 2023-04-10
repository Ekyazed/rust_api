use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE8_URL must be set");
    MysqlConnection::establish(&database_url).unwrap_or_else(|e| panic!("Error connecting to {}.\n this is the following error {}", database_url, e))
}