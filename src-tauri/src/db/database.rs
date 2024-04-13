use std::env;

use diesel::prelude::*;
use dotenvy::dotenv;
use log::info;

pub fn establish_connection() -> SqliteConnection {
    info!("started to establish sqlite connection",);

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to the database, {}",
                                                                         database_url))
}