mod schema;
mod models;

use diesel::sqlite::SqliteConnection;
use r2d2::{ Pool, Config };
use r2d2_diesel::ConnectionManager;
use dotenv::dotenv;
use std::env;

pub fn create_db_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let config = Config::default();
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::new(config, manager).expect("Failed to create pool.")
}
