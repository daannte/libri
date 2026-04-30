use std::{env, net::SocketAddr, path::PathBuf, sync::Arc};

use shiori::build_axum_router;
use shiori_core::{App, db};
use tokio::net::TcpListener;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    shiori::config::tracing::init();

    let app = Arc::new(App {
        pool: Arc::new(db::create_pool(
            &dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        )),
        base_path: PathBuf::from(env::var("DEV_DIR").unwrap_or_else(|_| "/data".to_string())),
    });

    let axum_router = build_axum_router(app);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::info!("Shiori server starting on http://{}", addr);

    axum::serve(listener, axum_router).await.unwrap();
}
