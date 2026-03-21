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
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

pub fn mount() -> Router<AppState> {
    Router::new()
        .route("/media", post(upload_media))
        .nest("/{id}", Router::new().route("/", get(get_media)))
}

#[utoipa::path(post, path = "/api/v1/media", tag = "media")]
async fn upload_media(State(app): State<AppState>) -> APIResult<Json<()>> {
    let _conn = app.db().await?;

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
