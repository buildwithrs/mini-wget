use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DLErrors {
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("response error: {0}")]
    RespError(String),

    #[error("invalid cl: {0}")]
    InvalidContentLength(#[from] ParseIntError),
}
