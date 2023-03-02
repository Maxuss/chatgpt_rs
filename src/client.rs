use std::str::FromStr;

use reqwest::header::AUTHORIZATION;
use reqwest::{
    header::{HeaderMap, HeaderValue, USER_AGENT},
    Method, Url,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::types::{ChatMessage, CompletionRequest, CompletionResponse, Role};

/// The client that operates the ChatGPT API
#[derive(Debug, Clone)]
pub struct ChatGPT {
    client: reqwest::Client,
    api_key: String,
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
        Ok(Self { client, api_key })
    }

    pub async fn send_simple_message<S: Into<String>>(
        &mut self,
        message: S,
    ) -> crate::Result<CompletionResponse> {
        self.client
            .post(
                Url::from_str("https://api.openai.com/v1/chat/completions")
                    .map_err(|err| crate::err::Error::ParsingError(err.to_string()))?,
            )
            .json(&CompletionRequest {
                model: "gpt-3.5-turbo",
                messages: vec![ChatMessage {
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
