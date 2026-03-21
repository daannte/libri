use diesel::HasQuery;
use utoipa::ToSchema;

#[derive(Debug, HasQuery, ToSchema)]
#[diesel(table_name = crate::schema::media)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Media {
    /// Unique identifier for the media item.
    pub id: i32,
    /// Name of the media file, excluding extension.
    pub name: String,
    /// Size of the media file in bytes.
    pub size: i32,
    /// File system path where the media is stored.
    pub path: String,
    /// The extension of the media.
    pub extension: String,
}
