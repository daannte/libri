use crate::metadata::provider::MetadataProvider;

pub struct GoodreadsProvider;

impl MetadataProvider for GoodreadsProvider {
    fn search(id: &str) -> String {
        todo!()
    }
}
