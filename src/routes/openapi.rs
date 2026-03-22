use axum::Router;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

use crate::config::state::AppState;

use super::api;

#[derive(OpenApi)]
#[openapi(
    paths(
        api::v1::media::list_media,
        api::v1::media::upload_media,
        api::v1::media::get_media,
        api::v1::metadata::search_metadata
    ),
    info(title = "shiori", description = "TODO DESCRIPTION OF SHIORI",)
)]
struct ApiDoc;

pub fn mount() -> Router<AppState> {
    Router::new().merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
}
