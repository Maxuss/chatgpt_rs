use serde::{Deserialize, Serialize};

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
}

/// ResponseEnum that can contain the old response and the new response
#[derive(Debug, Clone, Deserialize)]
pub enum ResponseType {
    #[cfg(feature = "gpt3")]
    Old(OldCompletionResponse),
    New(CompletionResponse)
}

/// Container for the sent/received ChatGPT messages
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Role of message sender
    pub role: Role,
    /// Actual content of the message
    pub content: String,
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
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
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
    /// Old completion successfuly completed
    #[cfg(feature = "gpt3")]
    OldCompletion(OldCompletionResponse),

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

    /// OldCompletionResponse struct to Completion Response
    #[cfg(feature = "gpt3")]
    pub fn from_old(old: OldCompletionResponse) -> Self {
        let message_choices:Vec<MessageChoice> = old.message_choices.into_iter().map(|choice| MessageChoice {
            message: ChatMessage {
                role: Role::System,
                content: choice.text,
            },
            finish_reason: choice.finish_reason,
            index: choice.index,
        }).collect();
        Self {
            message_id: Some(old.message_id),
            created_timestamp: Some(old.created_timestamp),
            model: old.model,
            usage: TokenUsage {
                prompt_tokens: old.usage.prompt_tokens,
                completion_tokens: 0,
                total_tokens: old.usage.total_tokens,
            },
            message_choices,
        }
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

/// Request for gpt3
#[cfg(feature = "gpt3")]
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct OldCompletionRequest<'a> {
    /// The model to be used, currently `gpt-3.5-turbo`, but may change in future
    pub model: &'a str,
    /// The message
    pub prompt: String,
    /// The maximum number of tokens to generate in the completion.
    pub max_tokens: u32,
    /// Whether the message response should be gradually streamed
    pub stream: bool,
    /// Include the log probabilities on the logprobs most likely tokens, as well the chosen tokens. For example, if logprobs is 5, the API will return a list of the 5 most likely tokens. The API will always return the logprob of the sampled token, so there may be up to logprobs+1 elements in the response. The maximum value for logprobs is 5
    pub logprobs: Option<u32>,
    /// Up to 4 sequences where the API will stop generating further tokens. The returned text will not contain the stop sequence.
    pub stop: String,
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
}

/// A response struct received from the API after requesting a message completion
#[cfg(feature = "gpt3")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Deserialize)]
pub struct OldCompletionResponse {
    /// Unique ID of the message, but not in a UUID format.
    /// Example: `chatcmpl-6p5FEv1JHictSSnDZsGU4KvbuBsbu`
    #[serde(rename = "id")]
    pub message_id: String,
    /// The model that was used for this completion
    #[serde(rename = "created")]
    pub created_timestamp: u64,
    pub object: String,
    /// The model that was used for this completion
    pub model: String,
    /// Token usage of this completion
    pub usage: OldTokenUsage,
    /// Message choices for this response, guaranteed to contain at least one message response
    #[serde(rename = "choices")]
    pub message_choices: Vec<MessageChoiceOld>,
}

/// A message completion choice struct
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Deserialize)]
#[cfg(feature = "gpt3")]
pub struct MessageChoiceOld {
    /// The actual message
    pub text: String,
    /// The index of this message in the outer `message_choices` array
    pub index: u32,
    pub logprobs: Option<i32>,
    /// The reason completion was stopped
    pub finish_reason: String,
}

/// The token usage of a specific response
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Deserialize)]
#[cfg(feature = "gpt3")]
pub struct OldTokenUsage {
    /// Tokens spent on the prompt message (including previous messages)
    pub prompt_tokens: u32,
    /// Total amount of tokens used (`prompt_tokens + completion_tokens`)
    pub total_tokens: u32,
}