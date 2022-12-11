use futures_util::{Stream, StreamExt};
use uuid::Uuid;

use crate::{client::ChatGPT, types::ResponsePart};

/// A container for a chat conversation
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ChatConversation {
    pub(crate) conversation_id: Option<Uuid>,
    pub(crate) parent_message_id: Option<Uuid>,
}

impl ChatConversation {
    /// Sends a message into this conversation
    ///
    /// Example:
    /// ```rust
    /// # use chatgpt::prelude::*;
    /// # use chatgpt::client::ChatGPT;
    /// # #[tokio::main]
    /// # async fn main() -> chatgpt::Result<()> {
    /// # let mut client = ChatGPT::new(std::env::var("SESSION_TOKEN").unwrap())?;
    /// # client.refresh_token().await?;
    /// let mut conversation = client.new_conversation();
    /// let response = conversation.send_message(&mut client, "Write me a sorting algorithm in Rust.").await?;
    /// println!("{response}");
    /// let response = conversation.send_message(&mut client, "Now can you rewrite it in Kotlin?").await?;
    /// println!("{response}");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_message<S: Into<String>>(
        &mut self,
        client: &ChatGPT,
        message: S,
    ) -> crate::Result<String> {
        let response = client
            .send_message_full(self.parent_message_id, self.conversation_id, message)
            .await?;
        if let Some(id) = response.conversation_id {
            self.conversation_id = Some(id);
        }

        self.parent_message_id = Some(response.message.id);

        Ok(response.message.content.parts[0].to_owned())
    }

    /// Sends a message into this conversation, returning the result as a stream.
    ///
    /// Example:
    /// ```rust
    /// # use chatgpt::prelude::*;
    /// # use chatgpt::client::ChatGPT;
    /// # #[tokio::main]
    /// # async fn main() -> chatgpt::Result<()> {
    /// # let mut client = ChatGPT::new(std::env::var("SESSION_TOKEN").unwrap())?;
    /// # client.refresh_token().await?;
    /// let mut conversation = client.new_conversation();
    /// let response = conversation.send_message(&mut client, "Write me a sorting algorithm in Rust.").await?;
    /// println!("{response}");
    /// let mut stream = conversation.send_message_streaming(&mut client, "Now can you rewrite it in Kotlin?").await?;
    /// while let Some(response) = stream.next().await {
    ///     println!("{response:?}");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_message_streaming<S: Into<String>>(
        &mut self,
        client: &ChatGPT,
        message: S,
    ) -> crate::Result<impl Stream<Item = crate::Result<ResponsePart>>> {
        let mut stream = client
            .send_message_streaming(self.parent_message_id, self.conversation_id, message)
            .await?;
        loop {
            // we are eating one iteration of messages off stream to update our conversation state
            match stream.next().await {
                Some(Ok(ResponsePart::Processing(response))) => {
                    if let Some(id) = response.conversation_id {
                        self.conversation_id = Some(id);
                    }

                    self.parent_message_id = Some(response.message.id);
                    break;
                }
                _ => continue,
            }
        }
        Ok(stream)
    }
}
