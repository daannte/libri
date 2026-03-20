use deadpool_diesel::sqlite::{Manager, Pool};

pub fn create_pool() -> Pool {
    let database_url = format!("file://{}/dev.db", env!("CARGO_MANIFEST_DIR"));

    let manager = Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);

    Pool::builder(manager).build().unwrap()
}
