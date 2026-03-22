pub use self::errors::MetadataError;
pub use self::goodreads::GoodreadsProvider;
pub use self::provider::MetadataProvider;

mod errors;
mod goodreads;
mod provider;
