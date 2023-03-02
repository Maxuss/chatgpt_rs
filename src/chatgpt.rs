#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

/// This module contains the ChatGPT client
pub mod client;
/// This module contains the errors related to the API
pub mod err;
/// The prelude module. Import everything from it to get the necessary elements from this library
pub mod prelude;
/// Types returned from the API and sent to it
pub mod types;

/// Result that is returned from most ChatGPT functions
pub type Result<T> = std::result::Result<T, err::Error>;

#[cfg(test)]
pub mod test {
    use crate::{client::ChatGPT, types::CompletionResponse};

    #[tokio::test]
    async fn test_client() -> crate::Result<()> {
        let mut client = ChatGPT::new(env!("TEST_API_KEY"))?;
        let resp = client.send_simple_message("Write me a short.").await?;
        println!("{resp:#?}");
        Ok(())
    }
}
