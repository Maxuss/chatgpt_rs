use std::string::FromUtf8Error;

use eventsource_stream::EventStreamError;
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
    /// A serde-provoked error has occurred
    #[error("Failed to (de)serialize data: {0}")]
    SerdeError(#[from] serde_json::Error),
    /// An error has occurred when parsing a string from UTF-8 bytes
    #[error("Failed to parse string from UTF-8: {0}")]
    StringError(#[from] FromUtf8Error),
    /// A backend related error has occurred
    #[error("An error occurred while processing request: {0}")]
    BackendError(String),
    /// An error has occurred when processing events over stream
    #[error("An error occurred while iterating over stream: {0}")]
    StreamError(#[from] EventStreamError<reqwest::Error>),
    /// An error has occurred, likely during simple authentication
    #[error("A fantoccini error has occurred: {0}")]
    #[cfg(feature = "simple-auth")]
    FantocciniWebError(#[from] fantoccini::error::WebDriver),
    /// An error has occurred, likely during simple authentication
    #[error("A fantoccini error has occurred: {0}")]
    #[cfg(feature = "simple-auth")]
    FantocciniCmdError(#[from] fantoccini::error::CmdError),
    /// An error has occurred, likely during simple authentication setup
    #[error("A fantoccini error has occurred: {0}")]
    #[cfg(feature = "simple-auth")]
    FantocciniSetupError(#[from] fantoccini::error::NewSessionError),
    /// An error has occurred while doing simple auth
    #[error("Simple authentication failed: {0}")]
    #[cfg(feature = "simple-auth")]
    SimpleAuthFailed(String),
}
