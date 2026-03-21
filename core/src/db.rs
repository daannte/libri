use std::env;

use diesel::SqliteConnection;
use diesel_async::{
    pooled_connection::{AsyncDieselConnectionManager, deadpool::Pool},
    sync_connection_wrapper::SyncConnectionWrapper,
};
use dotenvy::dotenv;

pub fn create_pool() -> Pool<SyncConnectionWrapper<SqliteConnection>> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = AsyncDieselConnectionManager::new(db_url);

    let pool = Pool::builder(manager).build().unwrap();

    pool
}
