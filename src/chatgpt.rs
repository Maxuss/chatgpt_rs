#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

/// This module contains the auth information
pub mod auth;
/// This module contains the ChatGPT client
pub mod client;
/// This module contains all the conversation logic
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
    use crate::{auth::simple::simple_auth, client::ChatGPT, types::ResponsePart};
    use futures_util::StreamExt;
    #[tokio::test]
    async fn test_client() {
        let email = std::env::var("EMAIL").unwrap();
        let password = std::env::var("PASSWORD").unwrap();

        let mut client = ChatGPT::new(simple_auth(email, password).unwrap()).unwrap();
        assert!(matches!(client.refresh_token().await, Ok(_)))
    }

    #[tokio::test]
    async fn test_message() -> crate::Result<()> {
        let email = std::env::var("EMAIL").unwrap();
        let password = std::env::var("PASSWORD").unwrap();

        let mut client = ChatGPT::new(simple_auth(email, password).unwrap()).unwrap();
        client.refresh_token().await?;
        let response = client
            .send_message_full(None, None, "Write me a simple sorting algorithm in Rust")
            .await?;
        println!("{}", response.message.content.parts[0]);
        Ok(())
    }

    #[tokio::test]
    async fn test_message_streaming() -> crate::Result<()> {
        let email = std::env::var("EMAIL").unwrap();
        let password = std::env::var("PASSWORD").unwrap();

        let mut client = ChatGPT::new(simple_auth(email, password).unwrap()).unwrap();
        client.refresh_token().await?;
        let mut stream = client
            .send_message_streaming(None, None, "Write me a simple sorting algorithm in Rust")
            .await?;
        while let Some(element) = stream.next().await {
            let element = element?;
            println!("{element:#?}")
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_conversations() -> crate::Result<()> {
        let email = std::env::var("EMAIL").unwrap();
        let password = std::env::var("PASSWORD").unwrap();

        let mut client = ChatGPT::new(simple_auth(email, password).unwrap()).unwrap();
        client.refresh_token().await?;
        let mut conversation = client.new_conversation();
        let response = conversation
            .send_message(&client, "Write a simple sorting algorithm in Rust")
            .await?;
        println!("{response}");
        let response = conversation
            .send_message(&client, "Now can you rewrite it in Kotlin?")
            .await?;
        println!("{response}");
        Ok(())
    }

    #[tokio::test]
    async fn test_conversations_streaming() -> crate::Result<()> {
        let email = std::env::var("EMAIL").unwrap();
        let password = std::env::var("PASSWORD").unwrap();

        let mut client = ChatGPT::new(simple_auth(email, password).unwrap()).unwrap();
        client.refresh_token().await?;
        let mut conversation = client.new_conversation();
        let response = conversation
            .send_message(&client, "Write a simple sorting algorithm in Rust")
            .await?;
        println!("{response}");
        let mut stream = conversation
            .send_message_streaming(&client, "Now can you rewrite it in Kotlin?")
            .await?;
        while let Some(part) = stream.next().await {
            let response = part?;
            match response {
                ResponsePart::Processing(data) => {
                    println!("{}", data.message.content.parts[0]);
                }
                _ => continue,
            }
        }
        Ok(())
    }
}
