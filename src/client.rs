use std::path::Path;
use std::str::FromStr;

use chrono::Local;
use reqwest::header::AUTHORIZATION;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Url,
};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::converse::Conversation;
use crate::types::{ChatMessage, CompletionRequest, CompletionResponse, Role};

/// The client that operates the ChatGPT API
#[derive(Debug, Clone)]
pub struct ChatGPT {
    client: reqwest::Client,
}

impl ChatGPT {
    /// Constructs a new ChatGPT client with default client options
    pub fn new<S: Into<String>>(api_key: S) -> crate::Result<Self> {
        let api_key = api_key.into();
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_bytes(format!("Bearer {api_key}").as_bytes())?,
        );
        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()?;
        Ok(Self { client })
    }

    pub async fn restore_conversation<P: AsRef<Path>>(
        &self,
        file: P,
    ) -> crate::Result<Conversation> {
        let path = file.as_ref();
        if !path.exists() {
            return Err(crate::err::Error::ParsingError(
                "Conversation history JSON file does not exist".to_string(),
            ));
        }
        let mut file = File::open(path).await?;
        let mut buf = String::new();
        file.read_to_string(&mut buf).await?;
        Ok(Conversation::new_with_history(
            self.clone(),
            serde_json::from_str(&buf)?,
        ))
    }

    pub fn new_conversation(&self) -> Conversation {
        self.new_conversation_directed(format!("You are ChatGPT, an AI model developed by OpenAI. Answer as concisely as possible. Today is: {0}", Local::now().to_string()))
    }

    pub fn new_conversation_directed<S: Into<String>>(&self, direction_message: S) -> Conversation {
        Conversation::new(self.clone(), direction_message.into())
    }

    #[must_use = "Sends a message to ChatGPT and uses your tokens"]
    pub async fn send_history(
        &self,
        history: &Vec<ChatMessage>,
    ) -> crate::Result<CompletionResponse> {
        self.client
            .post(
                Url::from_str("https://api.openai.com/v1/chat/completions")
                    .map_err(|err| crate::err::Error::ParsingError(err.to_string()))?,
            )
            .json(&CompletionRequest {
                model: "gpt-3.5-turbo",
                messages: history,
            })
            .send()
            .await?
            .json()
            .await
            .map_err(crate::err::Error::from)
    }

    #[must_use = "Sends a message to ChatGPT and uses your tokens"]
    pub async fn send_simple_message<S: Into<String>>(
        &self,
        message: S,
    ) -> crate::Result<CompletionResponse> {
        self.client
            .post(
                Url::from_str("https://api.openai.com/v1/chat/completions")
                    .map_err(|err| crate::err::Error::ParsingError(err.to_string()))?,
            )
            .json(&CompletionRequest {
                model: "gpt-3.5-turbo",
                messages: &vec![ChatMessage {
                    role: Role::User,
                    content: message.into(),
                }],
            })
            .send()
            .await?
            .json()
            .await
            .map_err(crate::err::Error::from)
    }
}
