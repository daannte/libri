use std::sync::Arc;

use shiori_core::ShioriCore;

use crate::routes::build_axum_router;

mod config;
mod errors;
mod routes;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let core = ShioriCore::new();
    let state = Arc::new(core.get_app());

    let axum_router = build_axum_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, axum_router).await.unwrap();
}
