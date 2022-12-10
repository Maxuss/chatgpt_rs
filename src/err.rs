use std::string::FromUtf8Error;

use reqwest::header::InvalidHeaderValue;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("An error occurred when processing a request: {0}")]
    ClientError(#[from] reqwest::Error),
    #[error("Invalid configuration provided: {0}")]
    InvalidConfiguration(#[from] InvalidHeaderValue),
    #[error("Server returned errror: {0}")]
    BackendError(String),
    #[error("Parsing error has occurred: {0}")]
    ParsingError(String),
    #[error("Failed to (de)serialize data: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("Failed to parse string from UTF-8: {0}")]
    StringError(#[from] FromUtf8Error),
}
