use std::{env::VarError, string::FromUtf8Error};

use reqwest::header::InvalidHeaderValue;
use thiserror::Error;

/// An error enum, used in the Result
#[derive(Debug, Error)]
pub enum Error {
    /// A reqwest-provoked error has occurred
    #[error("An error occurred when processing a request: {0}")]
    ClientError(#[from] reqwest::Error),
    /// Invalid header configuration error. Probably because of the custom User-Agent header
    #[error("Invalid configuration provided: {0}")]
    InvalidConfiguration(#[from] InvalidHeaderValue),
    /// An error that occurred when parsing data, e.g. a UUID
    #[error("Parsing error has occurred: {0}")]
    ParsingError(String),
    /// A serde-provoked JSON error has occurred
    #[cfg(feature = "json")]
    #[error("Failed to (de)serialize data: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[cfg(feature = "postcard")]
    /// A postcard-provoked error has occurred
    #[error("Failed to (de)serialize data: {0}")]
    PostcardError(#[from] postcard::Error),
    /// An error has occurred when parsing a string from UTF-8 bytes
    #[error("Failed to parse string from UTF-8: {0}")]
    StringError(#[from] FromUtf8Error),
    /// A backend related error has occurred
    #[error("An error occurred while processing request: {0}")]
    BackendError(String),
    /// A Tokio IO error happened
    #[error("Error happened during an IO operation: {0}")]
    IOError(#[from] tokio::io::Error),
    /// Most likely env var not provided
    #[error("Error while trying to access an environment variable: {0}")]
    VarError(#[from] VarError),
}
