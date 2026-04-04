use std::path;

use axum::Json;
use serde::Deserialize;
use shiori_api_types::{EncodableDirectories, EncodableDirectory};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    config::state::AppState,
    errors::{APIError, APIResult},
};

pub fn mount() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(list_directories))
}

/// List filesystem directories.
#[utoipa::path(
    post,
    path = "/filesystem/directories/list",
    request_body = inline(FolderRequest),
    tag = "filesystem",
    responses(
        (status = 200, description = "Successfully listed directories", body = inline(EncodableDirectories)),
        (status = 400, description = "Invalid filesystem path"),
        (status = 404, description = "Directory does not exist"),
        (status = 500, description = "Internal server error")
    )
)]
async fn list_directories(
    Json(body): Json<FolderRequest>,
) -> APIResult<Json<EncodableDirectories>> {
    let path = path::Path::new(&body.path);

    if !path.exists() {
        return Err(APIError::NotFound(format!(
            "Directory does not exist: {}",
            path.display()
        )));
    } else if !path.is_absolute() {
        return Err(APIError::BadRequest(format!(
            "Path must be absolute: {}",
            path.display()
        )));
    } else if !path.is_dir() {
        return Err(APIError::BadRequest(format!(
            "Path must be a directory: {}",
            path.display()
        )));
    }

    let dirs = shiori_filesystem::common::list_directories(path)?;

    let res = EncodableDirectories {
        parent: path.parent().map(|p| p.to_string_lossy().to_string()),
        directories: dirs.into_iter().map(EncodableDirectory::from).collect(),
    };

    Ok(Json(res))
}

#[derive(Deserialize, utoipa::ToSchema)]
struct FolderRequest {
    /// Path of the directory to list its subdirectories.
    path: String,
}
