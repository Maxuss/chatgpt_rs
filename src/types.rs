use core::f32;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct SessionRefresh {
    pub user: User,
    pub expires: String,
    #[serde(rename = "accessToken")]
    pub access_token: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub image: String,
    pub picture: String,
    pub groups: Vec<String>,
    pub features: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum PossiblyError<T> {
    Error { error: String },
    Fine(T),
}

#[derive(Debug, Clone, Deserialize, PartialEq, PartialOrd)]
pub struct ConversationResponse {
    pub conversation_id: Option<Uuid>,
    pub message: Message,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Message {
    pub id: Uuid,
    pub content: MessageContent,
    pub role: Role,
    pub user: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    pub weight: f32,
    pub recipient: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub enum MessageContentType {
    Text,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct MessageContent {
    pub content_type: MessageContentType,
    pub parts: Vec<String>,
}
