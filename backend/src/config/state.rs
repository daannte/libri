use std::sync::Arc;

use deadpool_diesel::sqlite::Pool;

use crate::db;

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<Pool>,
}

impl AppState {
    pub fn new() -> AppState {
        let pool = Arc::new(db::create_pool());
        AppState { pool }
    }
}
