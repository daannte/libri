use std::sync::Arc;

use deadpool_diesel::sqlite::Pool;

use crate::db;

#[derive(Clone)]
pub struct App {
    pub pool: Arc<Pool>,
}

impl App {
    pub fn new() -> App {
        let pool = Arc::new(db::create_pool());
        App { pool }
    }
}
