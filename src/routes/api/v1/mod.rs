use utoipa_axum::router::OpenApiRouter;

pub mod auth;
pub mod filesystem;
pub mod libraries;
pub mod media;
pub mod metadata;
pub mod system;
pub mod tokens;

use crate::config::state::AppState;

pub fn mount() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .merge(auth::mount())
        .merge(filesystem::mount())
        .merge(libraries::mount())
        .merge(media::mount())
        .merge(metadata::mount())
        .merge(tokens::mount())
}

pub fn mount_public(app: AppState) -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .merge(auth::mount_public(app))
        .merge(system::mount())
}
