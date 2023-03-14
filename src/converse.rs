use std::{path::Path, sync::Arc};

use tokio::{fs::File, io::AsyncWriteExt};

use crate::{
    client::ChatGPT,
    types::{ChatMessage, CompletionResponse, Role},
};

pub struct Conversation {
    client: Arc<ChatGPT>,
    pub history: Vec<ChatMessage>,
}

impl Conversation {
    pub fn new(client: Arc<ChatGPT>, first_message: String) -> Self {
        Self {
            client,
            history: vec![ChatMessage {
                role: Role::System,
                content: first_message,
            }],
        }
    }

    #[must_use = "Sends a message to ChatGPT and uses your tokens"]
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

    pub async fn save_history_json<P: AsRef<Path>>(&self, to: P) -> crate::Result<()> {
        let path = to.as_ref();
        if path.exists() {
            tokio::fs::remove_file(path).await?;
        }
        let mut file = File::create(path).await?;
        file.write_all(&serde_json::to_vec(&self.history)?).await?;
        Ok(())
    }
}
