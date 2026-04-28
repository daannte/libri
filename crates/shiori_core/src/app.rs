use std::{path::PathBuf, sync::Arc};

use diesel_async::{
    AsyncPgConnection,
    pooled_connection::deadpool::{Object, Pool, PoolError},
};

type DeadpoolResult = Result<Object<AsyncPgConnection>, PoolError>;

#[derive(Clone)]
pub struct App {
    pub pool: Arc<Pool<AsyncPgConnection>>,
    pub base_path: PathBuf,
}

impl App {
    pub async fn db(&self) -> DeadpoolResult {
        self.pool.get().await
    }
}
