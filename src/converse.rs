use std::path::Path;

use tokio::{fs::File, io::AsyncWriteExt};

#[cfg(feature = "streams")]
use {crate::types::ResponseChunk, futures::Stream};

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

    /// Rollbacks the history by 1 message, removing the last sent and received message.
    pub fn rollback(&mut self) -> Option<ChatMessage> {
        let last = self.history.pop();
        self.history.pop();
        last
    }

    /// Sends the message to the ChatGPT API and returns the completion response.
    ///
    /// Execution speed depends on API response times.
    pub async fn send_message<S: Into<String>>(
        &mut self,
        message: S,
    ) -> crate::Result<CompletionResponse> {
        self.history.push(ChatMessage {
            role: Role::User,
            content: message.into(),
        });
        let resp = self.client.send_history(&self.history).await?;
        self.history.push(resp.message_choices[0].message.clone());
        Ok(resp)
    }

    /// Sends the message to the ChatGPT API and returns the completion response as stream.
    ///
    /// Note, that this method will not automatically save the received message to history, as
    /// it is returned in streamed chunks. You will have to collect them into chat message yourself.
    ///
    /// You can use [`ChatMessage::from_response_chunks`] for this
    ///
    /// Requires the `streams` crate feature.
    #[cfg(feature = "streams")]
    pub async fn send_message_streaming<S: Into<String>>(
        &mut self,
        message: S,
    ) -> crate::Result<impl Stream<Item = ResponseChunk>> {
        self.history.push(ChatMessage {
            role: Role::User,
            content: message.into(),
        });
        let stream = self.client.send_history_streaming(&self.history).await?;
        Ok(stream)
    }

    /// Saves the history to a local JSON file, that can be restored to a conversation at runtime later.
    #[cfg(feature = "json")]
    pub async fn save_history_json<P: AsRef<Path>>(&self, to: P) -> crate::Result<()> {
        let path = to.as_ref();
        if path.exists() {
            tokio::fs::remove_file(path).await?;
        }
        let mut file = File::create(path).await?;
        file.write_all(&serde_json::to_vec(&self.history)?).await?;
        Ok(())
    }

    /// Saves the history to a local postcard file, that can be restored to a conversation at runtime later.
    #[cfg(feature = "postcard")]
    pub async fn save_history_postcard<P: AsRef<Path>>(&self, to: P) -> crate::Result<()> {
        let path = to.as_ref();
        if path.exists() {
            tokio::fs::remove_file(path).await?;
        }
        let mut file = File::create(path).await?;
        file.write_all(&postcard::to_allocvec(&self.history)?)
            .await?;
        Ok(())
    }
}
