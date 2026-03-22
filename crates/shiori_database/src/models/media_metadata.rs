use diesel::prelude::*;
use utoipa::ToSchema;

use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::schema::media_metadata;
use serde::Serialize;

/// The model representing a row in the `media_metadata` database table.
#[derive(Debug, HasQuery, ToSchema, Serialize)]
#[diesel(table_name = media_metadata)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MediaMetadata {
    /// Unique identifier for the media item.
    pub id: i32,
    /// List of authors associated with the media item.
    pub authors: Vec<String>,
    /// Name of the publisher or publishing organization.
    pub publisher: Option<String>,
    /// International Standard Book Number (ISBN).
    /// Typically used for books.
    pub isbn: Option<String>,
    /// Language of the media content.
    pub language: Option<String>,
}

/// Represents a new media_metadata record insertable to the `media_metadata` table.
#[derive(Debug, Insertable)]
#[diesel(table_name = media_metadata)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMediaMetadata<'a> {
    pub authors: Vec<&'a str>,
    pub publisher: Option<&'a str>,
    pub isbn: Option<&'a str>,
    pub language: Option<&'a str>,
}

impl NewMediaMetadata<'_> {
    pub async fn insert(&self, conn: &mut AsyncPgConnection) -> QueryResult<MediaMetadata> {
        diesel::insert_into(media_metadata::table)
            .values(self)
            .returning(MediaMetadata::as_returning())
            .get_result(conn)
            .await
    }
}
