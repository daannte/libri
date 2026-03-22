use shiori_api_types::EncodableMetadataSearch;

use crate::errors::MetadataError;

#[allow(async_fn_in_trait)]
pub trait MetadataProvider {
    const URL: &str;

    async fn search(id: &str) -> Result<EncodableMetadataSearch, MetadataError>;
}
