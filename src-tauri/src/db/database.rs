use std::env;

use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

use dotenvy::dotenv;
use log::info;

pub async fn create_connection_pool() -> Result<Pool<Sqlite>, sqlx::Error> {
    info!("started to establish sqlite connection pool",);

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePoolOptions::new().connect(&database_url).await?;
    Ok(pool)
}
