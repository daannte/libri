use deadpool_diesel::sqlite::{Manager, Pool};

pub fn create_pool() -> Pool {
    let database_url = "file:///data/dev.db";

    let manager = Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);

    Pool::builder(manager).build().unwrap()
}
