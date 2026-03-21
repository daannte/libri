use crate::models::media::NewMedia;
use crate::schema::media;
use crate::{
    config::state::AppState,
    errors::{APIError, APIResult},
    models::media::Media,
};
use axum::Router;
use axum::{
    Json,
    extract::{Path, State},
    routing::{get, post},
};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tempfile::NamedTempFile;
use utoipa::ToSchema;

pub fn mount() -> Router<AppState> {
    Router::new().nest(
        "/media",
        Router::new()
            .route("/", post(upload_media))
            .route("/{id}", get(get_media)),
    )
}

#[allow(unused)]
#[derive(TryFromMultipart, ToSchema)]
struct UploadMediaRequest {
    #[schema(value_type = Vec<Object>)]
    files: Vec<FieldData<NamedTempFile>>,
}

#[utoipa::path(
    post,
    path = "/api/v1/media",
    tag = "media",
    request_body(
        content = UploadMediaRequest,
        description = "Media file",
        content_type = "multipart/form-data"
    ),
    responses(
        (status = 200, description = "Successfully added media"),
        (status = 400, description = "Invalid media file"),
        (status = 500, description = "Internal server error")
    )
)]
async fn upload_media(
    State(app): State<AppState>,
    TypedMultipart(files): TypedMultipart<UploadMediaRequest>,
) -> APIResult<Json<()>> {
    let mut conn = app.db().await?;

    for f in files.files {
        let file_name = f.metadata.file_name.as_deref().ok_or(APIError::BadRequest(
            "Uploaded media must have filenames.".to_string(),
        ))?;
        println!("Found {}", file_name);

        let new_media = NewMedia {
            name: file_name,
            size: 0,
            path: "/",
            extension: "epub",
        };

        new_media.insert(&mut conn).await?;
    }

    Err(APIError::NotImplemented)
}

#[utoipa::path(
    get,
    path = "/api/v1/media/{id}",
    tag = "media",
    params(
        ("id" = i32, Path, description = "The id of the media to get")
    ),
    responses(
        (status = 200, description = "Successfully fetched media", body = Media),
        (status = 404, description = "Media not found"),
        (status = 500, description = "Internal server error")
    )
)]
async fn get_media(Path(media_id): Path<i32>, State(app): State<AppState>) -> APIResult<()> {
    let mut conn = app.db().await?;

    let media = Media::query()
        .filter(media::id.eq(media_id))
        .first::<Media>(&mut conn)
        .await?;

    println!("{:?}", media);

    Err(APIError::NotImplemented)
}
