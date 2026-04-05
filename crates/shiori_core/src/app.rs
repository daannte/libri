use std::{env, path::PathBuf, sync::Arc};

use diesel_async::{
    AsyncPgConnection,
    pooled_connection::deadpool::{Object, Pool, PoolError},
};

use crate::db;

type DeadpoolResult = Result<Object<AsyncPgConnection>, PoolError>;

#[derive(Clone)]
pub struct App {
    pub pool: Arc<Pool<AsyncPgConnection>>,
    pub base_path: PathBuf,
}

impl App {
    #[allow(clippy::new_without_default)]
    pub fn new() -> App {
        let pool = Arc::new(db::create_pool());

        // use APP_BASE_DIR for dev
        let base_path = env::var("APP_BASE_DIR").unwrap_or_else(|_| "/data".to_string());

        let base_path = PathBuf::from(base_path);

        let base_path = base_path
            .canonicalize()
            .expect("Failed to canonicalize base directory");

        App { pool, base_path }
    }

    pub async fn db(&self) -> DeadpoolResult {
        self.pool.get().await
    }
}
