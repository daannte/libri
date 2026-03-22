use axum::Router;

pub mod media;
pub mod metadata;

use crate::config::state::AppState;

pub fn mount() -> Router<AppState> {
    Router::new().merge(media::mount()).merge(metadata::mount())
}
