use std::path::Path;

use crate::{
    client::ChatGPT,
    types::{ChatMessage, CompletionResponse, Role},
};

/// Stores a single conversation session, and automatically saves message history
pub struct Conversation {
    client: ChatGPT,
    /// All the messages sent and received, starting with the beginning system message
    pub history: Vec<ChatMessage>,
}

impl Conversation {
    /// Constructs a new conversation from an API client and the introductory message
    pub fn new(client: ChatGPT, first_message: String) -> Self {
        Self {
            client,
            history: vec![ChatMessage {
                role: Role::System,
                content: first_message,
            }],
        }
    }

    /// Constructs a new conversation from a pre-initialized chat history
    pub fn new_with_history(client: ChatGPT, history: Vec<ChatMessage>) -> Self {
        Self { client, history }
    }

    /// Sends the message to the ChatGPT API and returns the completion response.
    ///
    /// Execution speed depends on API response times.
    pub fn send_message<S: Into<String>>(
        &mut self,
        message: S,
    ) -> String {
        self.history.push(ChatMessage {
            role: Role::User,
            content: message.into(),
        });
        let resp = self.client.send_history(&self.history);
        self.history.push(ChatMessage {
            role: Role::User,
            content: resp.clone(),
        });
        resp
    }
}
