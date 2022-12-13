use std::str::FromStr;

use crate::auth::AuthenticationData;
use crate::converse::ChatConversation;
use crate::types::{ConversationResponse, ResponsePart, SessionRefresh};
use eventsource_stream::{EventStream, Eventsource};
use futures_util::Stream;
use futures_util::StreamExt;
use reqwest::{
    header::{HeaderMap, HeaderValue, USER_AGENT},
    Method, Url,
};
use serde_json::json;
use uuid::Uuid;

/// Options for the ChatGPT client
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ClientOptions {
    api_url: Url,
    backend_api_url: Url,
}

impl ClientOptions {
    /// Sets the default API url. Default URL is https://chat.openai.com/api
    pub fn with_api_url(mut self, url: Url) -> Self {
        self.api_url = url;
        self
    }

    /// Sets the default backend API url. This is different from [`Self::with_api_url`] and defaults to https://chat.openai.com/backend-api
    pub fn with_backend_api_url(mut self, backend_url: Url) -> Self {
        self.backend_api_url = backend_url;
        self
    }
}

impl Default for ClientOptions {
    fn default() -> Self {
        Self {
            api_url: Url::from_str("https://chat.openai.com/api/").unwrap(),
            backend_api_url: Url::from_str("https://chat.openai.com/backend-api/").unwrap(),
        }
    }
}

/// The client that operates the ChatGPT API
#[derive(Debug, Clone)]
pub struct ChatGPT {
    client: reqwest::Client,
    options: ClientOptions,
    auth: AuthenticationData,
}

impl ChatGPT {
    /// Constructs a new ChatGPT client with default client options
    pub fn new(auth_data: AuthenticationData) -> crate::Result<Self> {
        Self::with_options(auth_data, ClientOptions::default())
    }

    /// Constructs a new ChatGPT client with the specified client options
    pub fn with_options(
        auth_data: AuthenticationData,
        options: ClientOptions,
    ) -> crate::Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_bytes(auth_data.user_agent.as_bytes())?,
        );
        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()?;
        Ok(Self {
            client,
            options,
            auth: auth_data,
        })
    }

    /// Refresh the access token. It is recommended to run this command after creating the client
    pub async fn refresh_token(&mut self) -> crate::Result<String> {
        let refresh = self
            .client
            .get(
                self.options
                    .api_url
                    .join("auth/session")
                    .map_err(|err| crate::err::Error::ParsingError(err.to_string()))?,
            )
            .header(
                "Authorization",
                format!("Bearer {}", self.auth.access_token),
            )
            .header(
                "Cookie",
                format!(
                    "__Secure-next-auth.session-token={}; cf_clearance={}",
                    self.auth.access_token, self.auth.cf_token
                ),
            )
            .send()
            .await?
            .json::<SessionRefresh>()
            .await;
        match refresh {
            Ok(refresh) => {
                self.auth.access_token = refresh.access_token.clone();
                Ok(refresh.access_token)
            }
            Err(_) => {
                // the previous access token is valid
                Ok(self.auth.access_token.clone())
            }
        }
    }

    /// Sends a messages and gets ChatGPT response.
    ///
    /// Note that usually it takes the AI around ~10-30 seconds to respond because of how the backend API is implemented.
    /// Because of that, sometimes you might want to use [`Self::send_message_streaming()`]
    ///
    /// Example:
    /// ```rust
    /// # use chatgpt::client::ChatGPT;
    /// # #[tokio::main]
    /// # async fn main() -> chatgpt::Result<()> {
    /// # let mut client = ChatGPT::new(std::env::var("SESSION_TOKEN").unwrap())?;
    /// # client.refresh_token().await?;
    /// let message = "Write me a sorting algorithm in Rust.";
    /// let response: String = client.send_message(message).await?;
    /// println!("{response}");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_message<S: Into<String>>(&self, message: S) -> crate::Result<String> {
        self.send_message_full(None, None, message)
            .await
            .map(|value| value.message.content.parts[0].to_owned())
    }

    /// Sends a message with parent message id and conversation id for conversations.
    ///
    /// Note that usually it takes the AI around ~10-30 seconds to respond because of how the backend API is implemented.
    /// Because of that, sometimes you might want to use [`Self::send_message_streaming()`]
    ///
    /// Example:
    /// ```rust
    /// # use chatgpt::prelude::*;
    /// # use chatgpt::client::ChatGPT;
    /// # #[tokio::main]
    /// # async fn main() -> chatgpt::Result<()> {
    /// # let mut client = ChatGPT::new(std::env::var("SESSION_TOKEN").unwrap())?;
    /// # client.refresh_token().await?;
    /// let message = "Write me a sorting algorithm in Rust.";
    /// let response: ConversationResponse = client.send_message_full(None, Some(uuid::Uuid::new_v4()), message).await?;
    /// println!("{response:?}");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_message_full<S: Into<String>>(
        &self,
        parent_message_id: Option<Uuid>,
        conversation_id: Option<Uuid>,
        message: S,
    ) -> crate::Result<ConversationResponse> {
        let mut stream = self
            .acquire_response_stream(parent_message_id, conversation_id, message.into())
            .await?;
        let mut last: String = "null".to_owned();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?.data;
            if chunk == "[DONE]" {
                break;
            } else {
                last = chunk;
            }
        }
        serde_json::from_str(&last).map_err(crate::err::Error::from)
    }

    /// Sends a message with full configuration and returns a stream of gradually finishing message
    ///
    /// Example:
    /// ```rust
    /// # use chatgpt::types::ResponsePart;
    /// # use chatgpt::client::ChatGPT;
    /// # use futures_util::StreamExt;
    /// # #[tokio::main]
    /// # async fn main() -> chatgpt::Result<()> {
    /// # let mut client = ChatGPT::new(std::env::var("SESSION_TOKEN").unwrap())?;
    /// # client.refresh_token().await?;
    /// let message = "Write me a sorting algorithm in Rust.";
    /// let mut stream = client.send_message_streaming(None, None, message).await?;
    /// while let Some(message) = stream.next().await {
    ///     match message? {
    ///         ResponsePart::PartialData => {
    ///             println!("Partial data received!")
    ///         }
    ///         ResponsePart::Processing(data) => {
    ///             println!("Got part of data: {data:?}");
    ///         }
    ///         ResponsePart::Done(data) => {
    ///             println!("Data processing finished! Response: {data:?}")
    ///         }
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_message_streaming<S: Into<String>>(
        &self,
        parent_message_id: Option<Uuid>,
        conversation_id: Option<Uuid>,
        message: S,
    ) -> crate::Result<impl Stream<Item = crate::Result<ResponsePart>>> {
        let stream = self
            .acquire_response_stream(parent_message_id, conversation_id, message.into())
            .await?;

        let mut collector: String = String::with_capacity(256);
        Ok(stream.map(move |part| {
            let chunk = part?.data;
            if chunk == "[DONE]" {
                crate::Result::Ok(ResponsePart::Done(serde_json::from_str(&collector)?))
            } else {
                collector = chunk;
                crate::Result::Ok(ResponsePart::Processing(serde_json::from_str(&collector)?))
            }
        }))
    }

    /// Begins a new scoped conversation
    pub fn new_conversation(&self) -> ChatConversation {
        ChatConversation {
            conversation_id: None,
            parent_message_id: None,
        }
    }

    async fn acquire_response_stream(
        &self,
        parent_message_id: Option<Uuid>,
        conversation_id: Option<Uuid>,
        message: String,
    ) -> crate::Result<EventStream<impl Stream<Item = reqwest::Result<bytes::Bytes>>>> {
        let mut body = json!({
            "action": "next",
            "messages": [
                {
                    "id": Uuid::new_v4(),
                    "role": "user",
                    "content": {
                        "content_type": "text",
                        "parts": [message]
                    }
                }
            ],
            "model": "text-davinci-002-render",
            "parent_message_id": parent_message_id.unwrap_or_else(Uuid::new_v4),
        });
        if let Some(id) = conversation_id {
            body.as_object_mut()
                .unwrap()
                .insert("conversation_id".into(), serde_json::to_value(id).unwrap());
        }
        Ok(self
            .client
            .request(
                Method::POST,
                self.options
                    .backend_api_url
                    .join("conversation")
                    .map_err(|err| crate::err::Error::ParsingError(err.to_string()))?,
            )
            .header(
                "Authorization",
                format!("Bearer {}", self.auth.access_token),
            )
            .header(
                "Cookie",
                format!(
                    "__Secure-next-auth.session-token={}; cf_clearance={}",
                    self.auth.access_token, self.auth.cf_token
                ),
            )
            .json(&body)
            .send()
            .await?
            .bytes_stream()
            .eventsource())
    }
}
