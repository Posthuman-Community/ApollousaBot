use diesel::{Connection, SqliteConnection};
use dotenvy::dotenv;
use std::env;

pub mod action;
pub mod model;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_uri = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_uri)
        .unwrap_or_else(|_| panic!("Error connecting to {database_uri}"))
}
