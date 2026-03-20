use std::sync::Arc;

use axum::Router;
use shiori_core::ShioriCore;

mod config;
mod routes;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let core = ShioriCore::new();
    let state = Arc::new(core.get_app());

    let app = Router::new()
        .merge(routes::mount())
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
