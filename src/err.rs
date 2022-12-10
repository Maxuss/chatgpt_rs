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
}
