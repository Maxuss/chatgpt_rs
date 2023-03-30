#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

/// This module contains the ChatGPT client
pub mod client;
/// This module contains additional configuration for ChatGPT
pub mod config;
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
    use std::path::Path;

    use futures::StreamExt;

    use crate::{
        client::ChatGPT,
        config::{ChatGPTEngine, ModelConfiguration},
        types::ResponseChunk,
    };

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
    async fn test_old_client() -> crate::Result<()> {
        let client = ChatGPT::oldnew(std::env::var("TEST_API_KEY")?)?;
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
        let _resp_a = conv.send_message("Could you tell me who you are?").await?;
        let _resp_b = conv
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
        let _resp = conv
            .send_message("Could you tell me what did I ask you about in my first question?")
            .await?;
        conv.save_history_json("history.json").await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_some_config() -> crate::Result<()> {
        let client = ChatGPT::new_with_config(
            std::env::var("TEST_API_KEY")?,
            ModelConfiguration::default()
                .temperature(0.9)
                .reply_count(3)
                .build(),
        )?;
        let response = client
            .send_message("Could you give me names of three popular Rust web frameworks?")
            .await?;
        assert!(response.message_choices.len() == 3);
        Ok(())
    }

    #[tokio::test]
    async fn test_streaming() -> crate::Result<()> {
        let client = ChatGPT::new(std::env::var("TEST_API_KEY")?)?;
        let response = client
            .send_message_streaming("Could you give me names of three popular Rust web frameworks?")
            .await?;
        let collected = response.collect::<Vec<ResponseChunk>>().await;
        assert_eq!(collected.last().unwrap().to_owned(), ResponseChunk::Done);
        Ok(())
    }

    #[tokio::test]
    async fn test_streaming_conv() -> crate::Result<()> {
        let client = ChatGPT::new(std::env::var("TEST_API_KEY")?)?;
        let mut conv = client.new_conversation();
        let _ = conv
            .send_message("Could you give me names of three popular Rust web frameworks?")
            .await?;
        let streamed = conv
            .send_message_streaming("Now could you do the same but for Kotlin?")
            .await?;
        let collected = streamed.collect::<Vec<ResponseChunk>>().await;
        assert_eq!(collected.last().unwrap().to_owned(), ResponseChunk::Done);
        Ok(())
    }
}
