use axum::Router;

use crate::config::state::AppState;

mod api;
mod koreader;
mod opds;
mod openapi;

pub fn build_axum_router(state: AppState) -> Router<()> {
    let router = Router::new();

    router
        .merge(api::mount())
        .merge(openapi::mount())
        .with_state(state)
}
