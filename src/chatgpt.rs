#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

/// This module contains the ChatGPT client
pub mod client;
/// Conversation related types
pub mod converse;
/// This module contains the errors related to the API
pub mod err;
/// The prelude module. Import everything from it to get the necessary elements from this library
pub mod prelude;
/// Types returned from the API and sent to it
pub mod types;

/// Result that is returned from most ChatGPT functions
pub type Result<T> = std::result::Result<T, err::Error>;