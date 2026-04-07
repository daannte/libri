use crate::{
    goodreads::{book, search},
    provider::{BooksParams, MetadataProvider, MetadataResult},
};
use shiori_api_types::EncodableMetadataSearch;

pub struct GoodreadsProvider;

impl MetadataProvider for GoodreadsProvider {
    const BOOK_URL: &str = "https://www.goodreads.com/book/show/";
    const SEARCH_URL: &str = "https://www.goodreads.com/search?search_type=books&q=";

    async fn search_books(params: BooksParams) -> MetadataResult<Vec<EncodableMetadataSearch>> {
        let ids = search::search_books(params).await?;

        let mut res = Vec::new();

        for id in ids {
            let book = Self::search_id(&id).await?;
            res.push(book);
        }

        Ok(res)
    }

    async fn search_id(id: &str) -> MetadataResult<EncodableMetadataSearch> {
        book::search_id(id).await
    }
}
