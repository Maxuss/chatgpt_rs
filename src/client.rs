use std::str::FromStr;

use reqwest::{
    header::{HeaderMap, HeaderValue, USER_AGENT},
    Method, Url,
};
use serde_json::json;
use uuid::Uuid;

use crate::types::{ConversationResponse, PossiblyError, SessionRefresh};

/// Options for the ChatGPT client
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ClientOptions {
    api_url: Url,
    backend_api_url: Url,
    user_agent: String,
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

    /// Sets the user agent for the client. Note that the API seems to filter out most of user agents except for default browser ones.
    pub fn with_user_agent<S: Into<String>>(mut self, user_agent: S) -> Self {
        self.user_agent = user_agent.into();
        self
    }
}

impl Default for ClientOptions {
    fn default() -> Self {
        Self {
            api_url: Url::from_str("https://chat.openai.com/api/").unwrap(),
            backend_api_url: Url::from_str("https://chat.openai.com/backend-api/").unwrap(),
            user_agent: "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36".into(),
        }
    }
}

/// The client that operates the ChatGPT API
#[derive(Debug, Clone)]
pub struct ChatGPT {
    client: reqwest::Client,
    options: ClientOptions,
    access_token: String,
}

impl ChatGPT {
    /// Constructs a new ChatGPT client with default client options
    pub fn new<S: Into<String>>(token: S) -> crate::Result<Self> {
        Self::with_options(token, ClientOptions::default())
    }

    /// Constructs a new ChatGPT client with the specified client options
    pub fn with_options<S: Into<String>>(token: S, options: ClientOptions) -> crate::Result<Self> {
        let token = token.into();
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_bytes(options.user_agent.as_bytes())?,
        );
        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .cookie_store(true)
            .build()?;
        Ok(Self {
            client,
            options,
            access_token: token,
        })
    }

    /// Refresh the access token. It is recommended to run this command after creating the client
    pub async fn refresh_token(&mut self) -> crate::Result<String> {
        let refresh: PossiblyError<SessionRefresh> = self
            .client
            .get(
                self.options
                    .api_url
                    .join("auth/session")
                    .map_err(|err| crate::err::Error::ParsingError(err.to_string()))?,
            )
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header(
                "Cookie",
                format!("__Secure-next-auth.session-token={}", self.access_token),
            )
            .send()
            .await?
            .json()
            .await?;
        match refresh {
            PossiblyError::Error { error } => Err(crate::err::Error::BackendError(error)),
            PossiblyError::Fine(refresh) => {
                self.access_token = refresh.access_token.clone();
                Ok(refresh.access_token)
            }
        }
    }

    /// Sends a messages and gets ChatGPT response. Note that usually it takes the AI around ~30 seconds to respond because of how the backend API is implemented.
    pub async fn send_message<S: Into<String>>(&mut self, message: S) -> crate::Result<String> {
        self.send_message_full(None, None, message)
            .await
            .map(|value| value.message.content.parts[0].to_owned())
    }

    /// Sends message with parent message id and conversation id for conversations. Note that usually it takes the AI around ~30 seconds to respond because of how the backend API is implemented.
    pub async fn send_message_full<S: Into<String>>(
        &mut self,
        parent_message_id: Option<Uuid>,
        conversation_id: Option<Uuid>,
        message: S,
    ) -> crate::Result<ConversationResponse> {
        let mut body = json!({
            "action": "next",
            "messages": [
                {
                    "id": Uuid::new_v4(),
                    "role": "user",
                    "content": {
                        "content_type": "text",
                        "parts": [message.into()]
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
        let mut stream = self
            .client
            .request(
                Method::POST,
                self.options
                    .backend_api_url
                    .join("conversation")
                    .map_err(|err| crate::err::Error::ParsingError(err.to_string()))?,
            )
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header(
                "Cookie",
                format!("__Secure-next-auth.session-token={}", self.access_token),
            )
            .json(&body)
            .send()
            .await?
            .bytes_stream();
        let mut last: String = "null".to_owned();

        use futures_util::StreamExt;
        while let Some(chunk) = stream.next().await {
            let chunk = String::from_utf8(chunk?.to_vec())?
                .replace("data: ", "")
                .to_owned();
            let chunk = chunk.trim().to_owned();
            if chunk == "[DONE]" {
                break;
            } else if !chunk.starts_with('{') {
                last += &chunk;
            } else {
                last = chunk;
            }
        }
        serde_json::from_str(&last).map_err(crate::err::Error::from)
    }
}
