pub mod downloader;
pub mod errors;

pub use downloader::Downloader;
pub use errors::DLErrors;

pub type DLResult<T> = Result<T, DLErrors>;
