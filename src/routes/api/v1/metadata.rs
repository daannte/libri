use axum::{Json, Router, extract::Query, routing::get};
use serde::Deserialize;
use shiori_api_types::EncodableMetadataSearch;
use shiori_metadata::{GoodreadsProvider, MetadataProvider};
use utoipa::IntoParams;

use crate::{config::state::AppState, errors::APIResult};

pub fn mount() -> Router<AppState> {
    Router::new().nest(
        "/metadata",
        Router::new().route("/search", get(search_metadata)),
    )
}

#[utoipa::path(
    get,
    path = "/api/v1/metadata/search",
    params(ListQueryParams),
    tag = "metadata",
    responses(
        (status = 200, description = "Successfully found metadata"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn search_metadata(
    Query(params): Query<ListQueryParams>,
) -> APIResult<Json<EncodableMetadataSearch>> {
    let metadata = GoodreadsProvider::search(&params.q_string).await?;
    Ok(Json(metadata))
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListQueryParams {
    /// A search query string.
    #[serde(rename = "q")]
    #[param(inline)]
    q_string: String,
}
