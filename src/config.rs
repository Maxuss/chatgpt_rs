use std::fmt::Display;

use derive_builder::Builder;

/// The struct containing main configuration for the ChatGPT API
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Builder)]
#[builder(default, setter(into))]
pub struct ModelConfiguration {
    /// The GPT version used.
    pub engine: ChatGPTEngine,
    /// Controls randomness of the output. Higher valeus means more random
    pub temperature: f32,
    /// Controls diversity via nucleus sampling, not recommended to use with temperature
    pub top_p: f32,
    /// Determines how much to penalize new tokens pased on their existing presence so far
    pub presence_penalty: f32,
    /// Determines how much to penalize new tokens based on their existing frequency so far
    pub frequency_penalty: f32,
    /// The maximum amount of replies
    pub reply_count: u32,
    /// URL of the /v1/chat/completions endpoint. Can be used to set a proxy
    pub api_url: &'static str,
}

impl Default for ModelConfiguration {
    fn default() -> Self {
        Self {
            engine: Default::default(),
            temperature: 0.5,
            top_p: 1.0,
            presence_penalty: 0.0,
            frequency_penalty: 0.0,
            reply_count: 1,
            api_url: "https://api.openai.com/v1/chat/completions",
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Builder)]
#[builder(default, setter(into))]
pub struct OldModelConfiguration {
    /// The GPT version used.
    pub engine: OldChatGPTEngine,
    /// Controls randomness of the output. Higher valeus means more random
    pub temperature: f32,
    /// Set the maximum number of tokens to generate
    pub max_tokens: u32,
    /// Controls diversity via nucleus sampling, not recommended to use with temperature
    pub top_p: f32,
    /// The maximum amount of replies
    pub reply_count: u32,
    /// Determines how much to penalize new tokens pased on their existing presence so far
    pub presence_penalty: f32,
    /// Determines how much to penalize new tokens based on their existing frequency so far
    pub frequency_penalty: f32,
    /// URL of the /v1/completions endpoint. Can be used to set a proxy
    pub api_url: &'static str,
    /// Include the log probabilities on the logprobs most likely tokens, as well the chosen tokens. For example, if logprobs is 5, the API will return a list of the 5 most likely tokens. The API will always return the logprob of the sampled token, so there may be up to logprobs+1 elements in the response. The maximum value for logprobs is 5
    pub logprobs: Option<u32>,
    /// Up to 4 sequences where the API will stop generating further tokens. The returned text will not contain the stop sequence.
    pub stop: String,
}

impl Default for OldModelConfiguration {
    fn default() -> Self {
        Self {
            engine: Default::default(),
            temperature: 0.5,
            max_tokens: 1024,
            top_p: 1.0,
            presence_penalty: 0.0,
            frequency_penalty: 0.0,
            reply_count: 1,
            api_url: "https://api.openai.com/v1/completions",
            logprobs: None,
            stop: "\n".to_string(),
        }
    }
}

/// The engine version for ChatGPT
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub enum ChatGPTEngine {
    /// Standard engine: `gpt-3.5-turbo`
    #[default]
    Gpt35Turbo,
    /// Different version of standard engine: `gpt-3.5-turbo-0301`
    Gpt35Turbo_0301,
    /// Base GPT-4 model: `gpt-4`
    Gpt4,
    /// Version of GPT-4, able to remember 32,000 tokens: `gpt-4-32k`
    Gpt4_32k,
    /// Different version of GPT-4: `gpt-4-0314`
    Gpt4_0314,
    /// Different version of GPT-4, able to remember 32,000 tokens: `gpt-4-32k-0314`
    Gpt4_32k_0314,
    /// Custom (or new/unimplemented) version of ChatGPT
    Custom(&'static str),
}

impl Display for ChatGPTEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub enum OldChatGPTEngine {
    /// Standard engine: `text-davinci-003`
    #[default]
    text_davinci_003,
    Custom(&'static str),
}

impl Display for OldChatGPTEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl AsRef<str> for ChatGPTEngine {
    fn as_ref(&self) -> &'static str {
        match self {
            ChatGPTEngine::Gpt35Turbo => "gpt-3.5-turbo",
            ChatGPTEngine::Gpt35Turbo_0301 => "gpt-3.5-turbo-0301",
            ChatGPTEngine::Gpt4 => "gpt-4",
            ChatGPTEngine::Gpt4_32k => "gpt-4-32k",
            ChatGPTEngine::Gpt4_0314 => "gpt-4-0314",
            ChatGPTEngine::Gpt4_32k_0314 => "gpt-4-32k-0314",
            ChatGPTEngine::Custom(custom) => custom,
        }
    }
}

impl AsRef<str> for OldChatGPTEngine {
    fn as_ref(&self) -> &'static str {
        match self {
            OldChatGPTEngine::text_davinci_003 => "text-davinci-003",
            OldChatGPTEngine::Custom(custom) => custom,
        }
    }
}