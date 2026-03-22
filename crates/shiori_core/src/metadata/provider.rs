pub trait MetadataProvider {
    fn search(id: &str) -> String;
}
