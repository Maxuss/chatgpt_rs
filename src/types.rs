use serde::{Deserialize, Serialize};
#[cfg(feature = "functions")]
use crate::functions::FunctionCall;

/// A role of a message sender, can be:
/// - `System`, for starting system message, that sets the tone of model
/// - `Assistant`, for messages sent by ChatGPT
/// - `User`, for messages sent by user
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// A system message, automatically sent at the start to set the tone of the model
    System,
    /// A message sent by ChatGPT
    Assistant,
    /// A message sent by the user
    User,
    /// A message related to ChatGPT functions. Does not have much use without the `functions` feature.
    Function,
}

/// Container for the sent/received ChatGPT messages
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Role of message sender
    pub role: Role,
    /// Actual content of the message
    pub content: String,
    /// Possibly function call that was attempted by the API.
    #[cfg(feature = "functions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCall>,
}

impl ChatMessage {
    /// Converts multiple response chunks into multiple (or a single) chat messages
    #[cfg(feature = "streams")]
    pub fn from_response_chunks(chunks: Vec<ResponseChunk>) -> Vec<Self> {
        let mut result: Vec<Self> = Vec::new();
        for chunk in chunks {
            match chunk {
                ResponseChunk::Content {
                    delta,
                    response_index,
                } => {
                    let msg = result
                        .get_mut(response_index)
                        .expect("Invalid response chunk sequence!");
                    msg.content.push_str(&delta);
                }
                ResponseChunk::BeginResponse {
                    role,
                    response_index: _,
                } => {
                    let msg = ChatMessage {
                        role,
                        content: String::new(),
                        #[cfg(feature = "functions")]
                        function_call: None
                    };
                    result.push(msg);
                }
                _ => {}
            }
        }
        result
    }
}

/// A request struct sent to the API to request a message completion
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CompletionRequest<'a> {
    /// The model to be used, currently `gpt-3.5-turbo`, but may change in future
    pub model: &'a str,
    /// The message history, including the message that requires completion, which should be the last one
    pub messages: &'a Vec<ChatMessage>,
    /// Whether the message response should be gradually streamed
    pub stream: bool,
    /// The extra randomness of response
    pub temperature: f32,
    /// Controls diversity via nucleus sampling, not recommended to use with temperature
    pub top_p: f32,
    /// Determines how much to penalize new tokens based on their existing frequency so far
    pub frequency_penalty: f32,
    /// Determines how much to penalize new tokens pased on their existing presence so far
    pub presence_penalty: f32,
    /// Determines the amount of output responses
    #[serde(rename = "n")]
    pub reply_count: u32,
    /// All functions that can be called by ChatGPT
    #[cfg(feature = "functions")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub functions: &'a Vec<serde_json::Value>
}

/// Represents a response from the API
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
#[serde(untagged)]
pub enum ServerResponse {
    /// An error occurred, most likely the model was just overloaded
    Error {
        /// The error that happened
        error: CompletionError,
    },
    /// Completion successfuly completed
    Completion(CompletionResponse),
}

/// An error happened while requesting completion
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct CompletionError {
    /// Message, describing the error
    pub message: String,
    /// The type of error. Example: `server_error`
    #[serde(rename = "type")]
    pub error_type: String,
}

/// A response struct received from the API after requesting a message completion
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct CompletionResponse {
    /// Unique ID of the message, but not in a UUID format.
    /// Example: `chatcmpl-6p5FEv1JHictSSnDZsGU4KvbuBsbu`
    #[serde(rename = "id")]
    pub message_id: Option<String>,
    /// Unix seconds timestamp of when the response was created
    #[serde(rename = "created")]
    pub created_timestamp: Option<u64>,
    /// The model that was used for this completion
    pub model: String,
    /// Token usage of this completion
    pub usage: TokenUsage,
    /// Message choices for this response, guaranteed to contain at least one message response
    #[serde(rename = "choices")]
    pub message_choices: Vec<MessageChoice>,
}

impl CompletionResponse {
    /// A shortcut to access the message response
    pub fn message(&self) -> &ChatMessage {
        // Unwrap is safe here, as we know that at least one message choice is provided
        &self.message_choices.first().unwrap().message
    }
}

/// A message completion choice struct
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct MessageChoice {
    /// The actual message
    pub message: ChatMessage,
    /// The reason completion was stopped
    pub finish_reason: String,
    /// The index of this message in the outer `message_choices` array
    pub index: u32,
}

/// The token usage of a specific response
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct TokenUsage {
    /// Tokens spent on the prompt message (including previous messages)
    pub prompt_tokens: u32,
    /// Tokens spent on the completion message
    pub completion_tokens: u32,
    /// Total amount of tokens used (`prompt_tokens + completion_tokens`)
    pub total_tokens: u32,
}

/// A single response chunk, returned from streamed request
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[cfg(feature = "streams")]
pub enum ResponseChunk {
    /// A chunk of message content
    Content {
        /// Piece of message content
        delta: String,
        /// Index of the message. Used when `reply_count` is set to more than 1 in API config
        response_index: usize,
    },
    /// Marks beginning of a new message response, with no actual content yet
    BeginResponse {
        /// The respondent's role (usually `Assistant`)
        role: Role,
        /// Index of the message. Used when `reply_count` is set to more than 1 in API config
        response_index: usize,
    },
    /// Ends a single message response response
    CloseResponse {
        /// Index of the message finished. Used when `reply_count` is set to more than 1 in API config
        response_index: usize,
    },
    /// Marks end of stream
    Done,
}

/// A part of a chunked inbound response
#[derive(Debug, Clone, Deserialize)]
#[cfg(feature = "streams")]
pub struct InboundResponseChunk {
    /// All message chunks in this response part (only one usually)
    pub choices: Vec<InboundChunkChoice>,
}

/// A single message part of a chunked inbound response
#[derive(Debug, Clone, Deserialize)]
#[cfg(feature = "streams")]
pub struct InboundChunkChoice {
    /// The part value of the response
    pub delta: InboundChunkPayload,
    /// Index of the message this chunk refers to
    pub index: usize,
}

/// Contains different chunked inbound response payloads
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
#[cfg(feature = "streams")]
pub enum InboundChunkPayload {
    /// Begins a single message by announcing roles (usually `assistant`)
    AnnounceRoles {
        /// The announced role
        role: Role,
    },
    /// Streams a part of message content
    StreamContent {
        /// The part of content
        content: String,
    },
    /// Closes a single message
    Close {},
}
