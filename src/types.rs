use core::f32;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Response that is received on the status refresh endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct SessionRefresh {
    /// User, to whom the token belongs
    pub user: User,
    /// Date when the token expires
    pub expires: String,
    /// The new refreshed token
    #[serde(rename = "accessToken")]
    pub access_token: String,
}

/// A container for User when returned from the session refresh
#[derive(Debug, Clone, Deserialize)]
pub struct User {
    /// Unique ID of this user
    pub id: String,
    /// Username of this user
    pub name: String,
    /// Email of this user
    pub email: String,
    /// Link to the avatar of this user. Usually a gravatar link
    pub image: String,
    /// Seems to be the same as the image field
    pub picture: String,
    /// Groups this user is in
    pub groups: Vec<String>,
    /// Special OpenAI features this user has
    pub features: Vec<String>,
}

/// A response that is received on the conversation endpoint
#[derive(Debug, Clone, Deserialize, PartialEq, PartialOrd)]
pub struct ConversationResponse {
    /// ID of the conversation this response belongs to
    pub conversation_id: Option<Uuid>,
    /// Message this conversation generated
    pub message: Message,
    /// Error (if present) that has occurred
    pub error: Option<String>,
}

/// The message that the user or the AI sent
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Message {
    /// Unique ID of the message
    pub id: Uuid,
    /// Content of this message
    pub content: MessageContent,
    /// Kind of sender. Either AI or user
    pub role: Role,
    /// The user that sent this message
    pub user: Option<String>,
    /// Creation time of this message
    pub create_time: Option<String>,
    /// Time at which this message was updated
    pub update_time: Option<String>,
    /// Weight for this message. The AI seems to return messages with 1.0 weight. The use for this field is unknown
    pub weight: f32,
    /// Recipient, who was this message sent to
    pub recipient: String,
}

/// Kind of content in the message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub enum MessageContentType {
    /// A simple text message
    Text,
}

/// Kind of sender
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    /// A user sent this message
    User,
    /// An AI sent this message
    Assistant,
}

/// The content of the message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct MessageContent {
    /// Kind of message
    pub content_type: MessageContentType,
    /// The text parts of this message. The AI seems to always output the message in a single element array, as well as the user
    pub parts: Vec<String>,
}

/// Part of a mapped response returned from the [`ChatGPT::send_message_streaming()`](`chatgpt::client::ChatGPT::send_message_streaming()`) method
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ResponsePart {
    /// The data recieved eneded abruptly, needs another part of data to build a chunk of response
    PartialData,
    /// Got a chunk of response containing unfinished message response
    Processing(ConversationResponse),
    /// Got an indication that the final response was returned
    Done(ConversationResponse),
}
