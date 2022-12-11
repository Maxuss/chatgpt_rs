#[doc = include_str!("../README.md")]
pub mod client;
pub mod err;
pub mod prelude;
pub mod types;

pub type Result<T> = std::result::Result<T, err::Error>;

#[cfg(test)]
pub mod test {
    use crate::{client::ChatGPT, types::ResponsePart};
    use futures_util::StreamExt;

    #[tokio::test]
    async fn test_client() {
        let token = std::env::var("SESSION_TOKEN").unwrap();
        let mut client = ChatGPT::new(&token).unwrap();
        assert!(matches!(client.refresh_token().await, Ok(_)))
    }

    #[tokio::test]
    async fn test_message() {
        let token = std::env::var("SESSION_TOKEN").unwrap();
        let mut client = ChatGPT::new(&token).unwrap();
        client.refresh_token().await.unwrap();
        let response = client
            .send_message_full(None, None, "Write me a simple sorting algorithm in Rust")
            .await
            .unwrap();
        println!("{}", response.message.content.parts[0])
    }

    #[tokio::test]
    async fn test_message_streaming() {
        let token = std::env::var("SESSION_TOKEN").unwrap();
        let mut client = ChatGPT::new(&token).unwrap();
        client.refresh_token().await.unwrap();
        let mut stream = client
            .send_message_streaming(None, None, "Write me a simple sorting algorithm in Rust")
            .await
            .unwrap()
            .filter(|it| futures::future::ready(!matches!(it, Ok(ResponsePart::PartialData))));
        while let Some(element) = stream.next().await {
            let element = element.unwrap();
            println!("{element:#?}")
        }
    }
}
