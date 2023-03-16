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

#[cfg(test)]
pub mod test {
    use std::{fs::File, path::Path};

    use crate::client::ChatGPT;

    #[tokio::test]
    async fn test_client() -> crate::Result<()> {
        let client = ChatGPT::new(std::env::var("TEST_API_KEY")?)?;
        let resp = client
            .send_message("Write me a short pun about the Rust language.")
            .await?;
        assert!(!resp.message_choices.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_undirected_conversation() -> crate::Result<()> {
        let client = ChatGPT::new(std::env::var("TEST_API_KEY")?)?;
        let mut conv = client.new_conversation();
        let resp = conv
            .send_message("Could you tell me what day is it today?")
            .await?;
        assert!(!resp.message_choices.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_conversation() -> crate::Result<()> {
        let client = ChatGPT::new(std::env::var("TEST_API_KEY")?)?;
        let mut conv = client.new_conversation_directed(
            "You are TestGPT, an AI model developed in Rust in year 2023.",
        );
        let resp_a = conv.send_message("Could you tell me who you are?").await?;
        let resp_b = conv
            .send_message("What did I ask you about in my first question?")
            .await?;
        assert!(!resp_a.message_choices.is_empty() && !resp_b.message_choices.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_conversation_saving() -> crate::Result<()> {
        let client = ChatGPT::new(std::env::var("TEST_API_KEY")?)?;
        let mut conv = client.new_conversation_directed(
            "You are TestGPT, an AI model developed in Rust in year 2023.",
        );
        let resp_a = conv.send_message("Could you tell me who you are?").await?;
        let resp_b = conv
            .send_message("What did I ask you about in my first question?")
            .await?;
        conv.save_history_json("history.json").await?;
        let path: &Path = "history.json".as_ref();
        assert!(path.exists());
        Ok(())
    }

    #[tokio::test]
    async fn test_conversation_restoring() -> crate::Result<()> {
        let client = ChatGPT::new(std::env::var("TEST_API_KEY")?)?;
        let mut conv = client.restore_conversation_json("history.json").await?;
        let resp = conv
            .send_message("Could you tell me what did I ask you about in my first question?")
            .await?;
        conv.save_history_json("history.json").await?;
        Ok(())
    }
}
