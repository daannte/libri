use axum::Router;

pub mod media;

use crate::config::state::AppState;

pub fn mount() -> Router<AppState> {
    Router::new().merge(media::mount())
}
