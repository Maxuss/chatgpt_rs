pub mod client;
pub mod err;
pub mod types;

pub type Result<T> = std::result::Result<T, err::Error>;

#[cfg(test)]
pub mod test {
    use crate::client::ChatGPT;

    #[tokio::test]
    async fn test_client() {
        let token = std::env::var("SESSION_TOKEN").unwrap();
        let mut client = ChatGPT::new(&token).unwrap();
        assert!(matches!(client.refresh_token().await, Ok(_)))
    }
}
