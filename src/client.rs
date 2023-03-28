use std::path::Path;
use std::str::FromStr;
use ureq::{Agent, AgentBuilder};
use chrono::Local;
use reqwest::header::AUTHORIZATION;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Url,
};

use crate::converse::Conversation;
use crate::types::{ChatMessage, CompletionRequest, CompletionResponse, Role, ServerResponse};

/// The client that operates the ChatGPT API
#[derive(Debug, Clone)]
pub struct ChatGPT {
    client: ureq::Agent,
    token: String,
}

impl ChatGPT {
    /// Constructs a new ChatGPT API client with provided API Key
    pub fn new<S: Into<String>>(api_key: S) -> crate::Result<Self> {
        let api_key = api_key.into();
        let token = format!("Bearer {api_key}");
        let client = AgentBuilder::new().build();
        Ok(Self { client, token})
    }

    /// Starts a new conversation with a default starting message.
    ///
    /// Conversations record message history.
    pub fn new_conversation(&self) -> Conversation {
        self.new_conversation_directed(format!("You are ChatGPT, an AI model developed by OpenAI. Answer as concisely as possible. Today is: {0}", Local::now().format("%d/%m/%Y %H:%M")))
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
    pub fn send_history(
        &self,
        history: &Vec<ChatMessage>,
    ) -> String {
        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .set("Authorization", &self.token)
            .send_json(ureq::json!({
                "model": "gpt-3.5-turbo",
                "rust": true,
                "messages": history
            }))
            .unwrap()
            .into_string();
        response.unwrap_or_default()
    }

    /// Sends a single message to the API without preserving message history.
    pub fn send_message<S: Into<String>>(
        &self,
        message: S,
    ) -> Result<CompletionResponse, ureq::Error> {
        let response: CompletionResponse = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .set("Authorization", &self.token)
            .send_json(ureq::json!({
                "model": "gpt-3.5-turbo",
                "messages": &vec![ChatMessage {
                    role: Role::User,
                    content: message.into(),
                }],
            }))
            .unwrap()
            .into_json()?;
        Ok(response)
    }
}
