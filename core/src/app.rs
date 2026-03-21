use std::sync::Arc;

use diesel::SqliteConnection;
use diesel_async::{
    pooled_connection::deadpool::{Object, Pool, PoolError},
    sync_connection_wrapper::SyncConnectionWrapper,
};

use crate::db;

type DeadpoolResult = Result<Object<SyncConnectionWrapper<SqliteConnection>>, PoolError>;

#[derive(Clone)]
pub struct App {
    pub pool: Arc<Pool<SyncConnectionWrapper<SqliteConnection>>>,
}

impl App {
    pub fn new() -> App {
        let pool = Arc::new(db::create_pool());
        App { pool }
    }

    pub async fn db(&self) -> DeadpoolResult {
        self.pool.get().await
    }
}
