use std::sync::Arc;

use diesel::SqliteConnection;
use diesel_async::{
    pooled_connection::deadpool::{Object, Pool, PoolError},
    sync_connection_wrapper::SyncConnectionWrapper,
};

use crate::db;

type WrapperConn = SyncConnectionWrapper<SqliteConnection>;
pub type SqliteConn = Object<WrapperConn>;

type DeadpoolResult = Result<SqliteConn, PoolError>;

#[derive(Clone)]
pub struct App {
    pub pool: Arc<Pool<WrapperConn>>,
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
