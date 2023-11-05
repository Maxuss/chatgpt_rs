use std::path::Path;

use reqwest::header::AUTHORIZATION;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{self, Proxy};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[cfg(feature = "streams")]
use reqwest::Response;
#[cfg(feature = "streams")]
use {
    crate::types::InboundChunkPayload, crate::types::InboundResponseChunk,
    crate::types::ResponseChunk, futures_util::Stream,
};

use crate::config::ModelConfiguration;
use crate::converse::Conversation;
use crate::types::{ChatMessage, CompletionRequest, CompletionResponse, Role, ServerResponse};

#[cfg(feature = "functions")]
use crate::functions::{FunctionArgument, FunctionDescriptor};

/// The client that operates the ChatGPT API
#[derive(Debug, Clone)]
pub struct ChatGPT {
    client: reqwest::Client,
    /// The configuration for this ChatGPT client
    pub config: ModelConfiguration,
}

impl ChatGPT {
    /// Constructs a new ChatGPT API client with provided API key and default configuration
    pub fn new<S: Into<String>>(api_key: S) -> crate::Result<Self> {
        Self::new_with_config(api_key, ModelConfiguration::default())
    }

    /// Constructs a new ChatGPT API client with provided API key, default configuration and a reqwest proxy
    pub fn new_with_proxy<S: Into<String>>(api_key: S, proxy: Proxy) -> crate::Result<Self> {
        Self::new_with_config_proxy(api_key, ModelConfiguration::default(), proxy)
    }

    /// Constructs a new ChatGPT API client with provided API Key and Configuration
    pub fn new_with_config<S: Into<String>>(
        api_key: S,
        config: ModelConfiguration,
    ) -> crate::Result<Self> {
        let api_key = api_key.into();
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_bytes(format!("Bearer {api_key}").as_bytes())?,
        );
        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .timeout(config.timeout)
            .build()?;
        Ok(Self { client, config })
    }

    /// Constructs a new ChatGPT API client with provided API Key, Configuration and Reqwest proxy
    pub fn new_with_config_proxy<S: Into<String>>(
        api_key: S,
        config: ModelConfiguration,
        proxy: Proxy,
    ) -> crate::Result<Self> {
        let api_key = api_key.into();
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_bytes(format!("Bearer {api_key}").as_bytes())?,
        );

        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .timeout(config.timeout)
            .proxy(proxy)
            .build()?;
        Ok(Self { client, config })
    }
    /// Restores a conversation from local conversation JSON file.
    /// The conversation file can originally be saved using the [`Conversation::save_history_json()`].
    #[cfg(feature = "json")]
    pub async fn restore_conversation_json<P: AsRef<Path>>(
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

    /// Restores a conversation from local conversation postcard file.
    /// The conversation file can originally be saved using the [`Conversation::save_history_postcard()`].
    #[cfg(feature = "postcard")]
    pub async fn restore_conversation_postcard<P: AsRef<Path>>(
        &self,
        file: P,
    ) -> crate::Result<Conversation> {
        let path = file.as_ref();
        if !path.exists() {
            return Err(crate::err::Error::ParsingError(
                "Conversation history Postcard file does not exist".to_string(),
            ));
        }
        let mut file = File::open(path).await?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).await?;
        Ok(Conversation::new_with_history(
            self.clone(),
            postcard::from_bytes(&buf)?,
        ))
    }

    /// Starts a new conversation with a default starting message.
    ///
    /// Conversations record message history.
    pub fn new_conversation(&self) -> Conversation {
        self.new_conversation_directed(
            "You are ChatGPT, an AI model developed by OpenAI. Answer as concisely as possible."
                .to_string(),
        )
    }

    /// Starts a new conversation with a specified starting message.
    ///
    /// Conversations record message history.
    pub fn new_conversation_directed<S: Into<String>>(&self, direction_message: S) -> Conversation {
        Conversation::new(self.clone(), direction_message.into())
    }

    /// Explicitly sends whole message history to the API.
    ///
    /// In most cases, if you would like to store message history, you should be looking at the [`Conversation`] struct, and
    /// [`Self::new_conversation()`] and [`Self::new_conversation_directed()`]
    pub async fn send_history(
        &self,
        history: &Vec<ChatMessage>,
    ) -> crate::Result<CompletionResponse> {
        let response: ServerResponse = self
            .client
            .post(self.config.api_url.clone())
            .json(&CompletionRequest {
                model: self.config.engine.as_ref(),
                messages: history,
                stream: false,
                temperature: self.config.temperature,
                top_p: self.config.top_p,
                max_tokens: self.config.max_tokens,
                frequency_penalty: self.config.frequency_penalty,
                presence_penalty: self.config.presence_penalty,
                reply_count: self.config.reply_count,
                #[cfg(feature = "functions")]
                functions: &Vec::new(),
            })
            .send()
            .await?
            .json()
            .await?;
        match response {
            ServerResponse::Error { error } => Err(crate::err::Error::BackendError {
                message: error.message,
                error_type: error.error_type,
            }),
            ServerResponse::Completion(completion) => Ok(completion),
        }
    }

    /// Explicitly sends whole message history to the API and returns the response as stream. **Stream will be empty** if
    /// any errors are returned from the server.
    ///
    /// In most cases, if you would like to store message history, you should be looking at the [`Conversation`] struct, and
    /// [`Self::new_conversation()`] and [`Self::new_conversation_directed()`]
    ///
    /// Requires the `streams` crate feature
    #[cfg(feature = "streams")]
    pub async fn send_history_streaming(
        &self,
        history: &Vec<ChatMessage>,
    ) -> crate::Result<impl Stream<Item = ResponseChunk>> {
        let response = self
            .client
            .post(self.config.api_url.clone())
            .json(&CompletionRequest {
                model: self.config.engine.as_ref(),
                stream: true,
                messages: history,
                temperature: self.config.temperature,
                top_p: self.config.top_p,
                max_tokens: self.config.max_tokens,
                frequency_penalty: self.config.frequency_penalty,
                presence_penalty: self.config.presence_penalty,
                reply_count: self.config.reply_count,
                #[cfg(feature = "functions")]
                functions: &Vec::new(),
            })
            .send()
            .await?;

        Self::process_streaming_response(response)
    }

    /// Sends a single message to the API without preserving message history.
    pub async fn send_message<S: Into<String>>(
        &self,
        message: S,
    ) -> crate::Result<CompletionResponse> {
        let response: ServerResponse = self
            .client
            .post(self.config.api_url.clone())
            .json(&CompletionRequest {
                model: self.config.engine.as_ref(),
                messages: &vec![ChatMessage {
                    role: Role::User,
                    content: message.into(),
                    #[cfg(feature = "functions")]
                    function_properties: None,
                }],
                stream: false,
                temperature: self.config.temperature,
                top_p: self.config.top_p,
                max_tokens: self.config.max_tokens,
                frequency_penalty: self.config.frequency_penalty,
                presence_penalty: self.config.presence_penalty,
                reply_count: self.config.reply_count,
                #[cfg(feature = "functions")]
                functions: &Vec::new(),
            })
            .send()
            .await?
            .json()
            .await?;
        match response {
            ServerResponse::Error { error } => Err(crate::err::Error::BackendError {
                message: error.message,
                error_type: error.error_type,
            }),
            ServerResponse::Completion(completion) => Ok(completion),
        }
    }

    /// Sends a single message to the API, and returns the response as stream, without preserving message history. **Stream will be empty** if
    /// any errors are returned from the server.
    ///
    /// Requires the `streams` crate feature
    #[cfg(feature = "streams")]
    pub async fn send_message_streaming<S: Into<String>>(
        &self,
        message: S,
    ) -> crate::Result<impl Stream<Item = ResponseChunk>> {
        let response = self
            .client
            .post(self.config.api_url.clone())
            .json(&CompletionRequest {
                model: self.config.engine.as_ref(),
                messages: &vec![ChatMessage {
                    role: Role::User,
                    content: message.into(),
                    #[cfg(feature = "functions")]
                    function_properties: None,
                }],
                stream: true,
                temperature: self.config.temperature,
                top_p: self.config.top_p,
                max_tokens: self.config.max_tokens,
                frequency_penalty: self.config.frequency_penalty,
                presence_penalty: self.config.presence_penalty,
                reply_count: self.config.reply_count,
                #[cfg(feature = "functions")]
                functions: &Vec::new(),
            })
            .send()
            .await?;

        Self::process_streaming_response(response)
    }

    #[cfg(feature = "streams")]
    fn process_streaming_response(
        response: Response,
    ) -> crate::Result<impl Stream<Item = ResponseChunk>> {
        use eventsource_stream::Eventsource;
        use futures_util::StreamExt;

        // also handles errors
        response
            .error_for_status()
            .map(|response| {
                let response_stream = response.bytes_stream().eventsource();
                response_stream.map(move |part| {
                    let chunk = &part.expect("Stream closed abruptly!").data;
                    if chunk == "[DONE]" {
                        return ResponseChunk::Done;
                    }
                    let data: InboundResponseChunk = serde_json::from_str(chunk)
                        .expect("Invalid inbound streaming response payload!");
                    let choice = data.choices[0].to_owned();
                    match choice.delta {
                        InboundChunkPayload::AnnounceRoles { role } => {
                            ResponseChunk::BeginResponse {
                                role,
                                response_index: choice.index,
                            }
                        }
                        InboundChunkPayload::StreamContent { content } => ResponseChunk::Content {
                            delta: content,
                            response_index: choice.index,
                        },
                        InboundChunkPayload::Close {} => ResponseChunk::CloseResponse {
                            response_index: choice.index,
                        },
                    }
                })
            })
            .map_err(crate::err::Error::from)
    }

    /// Sends a message with specified function descriptors. ChatGPT is then able to call these functions.
    ///
    /// **NOTE**: Functions are processed [as tokens on the backend](https://platform.openai.com/docs/guides/gpt/function-calling),
    /// so you might want to limit the amount of functions or their description.
    #[cfg(feature = "functions")]
    pub async fn send_message_functions<S: Into<String>, A: FunctionArgument>(
        &self,
        message: S,
        functions: Vec<FunctionDescriptor<A>>,
    ) -> crate::Result<CompletionResponse> {
        self.send_message_functions_baked(
            message,
            functions
                .into_iter()
                .map(serde_json::to_value)
                .collect::<serde_json::Result<Vec<serde_json::Value>>>()
                .map_err(crate::err::Error::from)?,
        )
        .await
    }

    /// Sends a message with specified pre-baked function descriptors. ChatGPT is then able to call these functions.
    ///
    /// **NOTE**: Functions are processed [as tokens on the backend](https://platform.openai.com/docs/guides/gpt/function-calling),
    /// so you might want to limit the amount of functions or their description.
    #[cfg(feature = "functions")]
    pub async fn send_message_functions_baked<S: Into<String>>(
        &self,
        message: S,
        baked_functions: Vec<serde_json::Value>,
    ) -> crate::Result<CompletionResponse> {
        let response: ServerResponse = self
            .client
            .post(self.config.api_url.clone())
            .json(&CompletionRequest {
                model: self.config.engine.as_ref(),
                messages: &vec![ChatMessage {
                    role: Role::User,
                    content: message.into(),
                    #[cfg(feature = "functions")]
                    function_properties: None,
                }],
                stream: false,
                temperature: self.config.temperature,
                top_p: self.config.top_p,
                frequency_penalty: self.config.frequency_penalty,
                presence_penalty: self.config.presence_penalty,
                reply_count: self.config.reply_count,
                max_tokens: self.config.max_tokens,
                #[cfg(feature = "functions")]
                functions: &baked_functions,
            })
            .send()
            .await?
            .json()
            .await?;

        match response {
            ServerResponse::Error { error } => Err(crate::err::Error::BackendError {
                message: error.message,
                error_type: error.error_type,
            }),
            ServerResponse::Completion(completion) => Ok(completion),
        }
    }

    /// Sends whole message history alongside with defined baked functions.
    #[cfg(feature = "functions")]
    pub async fn send_history_functions(
        &self,
        history: &Vec<ChatMessage>,
        functions: &Vec<serde_json::Value>,
    ) -> crate::Result<CompletionResponse> {
        let response: ServerResponse = self
            .client
            .post(self.config.api_url.clone())
            .json(&CompletionRequest {
                model: self.config.engine.as_ref(),
                messages: history,
                stream: false,
                temperature: self.config.temperature,
                top_p: self.config.top_p,
                frequency_penalty: self.config.frequency_penalty,
                presence_penalty: self.config.presence_penalty,
                reply_count: self.config.reply_count,
                max_tokens: self.config.max_tokens,
                functions,
            })
            .send()
            .await?
            .json()
            .await?;
        match response {
            ServerResponse::Error { error } => Err(crate::err::Error::BackendError {
                message: error.message,
                error_type: error.error_type,
            }),
            ServerResponse::Completion(completion) => Ok(completion),
        }
    }
}
