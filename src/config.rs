use std::{fmt::Display, str::FromStr};
use std::time::Duration;

#[cfg(feature = "functions")]
use crate::functions::FunctionValidationStrategy;
use derive_builder::Builder;

/// The struct containing main configuration for the ChatGPT API
#[derive(Debug, Clone, PartialEq, PartialOrd, Builder)]
#[builder(default, setter(into))]
pub struct ModelConfiguration {
    /// The GPT version used.
    pub engine: ChatGPTEngine,
    /// Controls randomness of the output. Higher values means more random
    pub temperature: f32,
    /// Controls diversity via nucleus sampling, not recommended to use with temperature
    pub top_p: f32,
    /// Controls the maximum number of tokens to generate in the completion
    pub max_tokens: Option<u32>,
    /// Determines how much to penalize new tokens passed on their existing presence so far
    pub presence_penalty: f32,
    /// Determines how much to penalize new tokens based on their existing frequency so far
    pub frequency_penalty: f32,
    /// The maximum amount of replies
    pub reply_count: u32,
    /// URL of the /v1/chat/completions endpoint. Can be used to set a proxy
    pub api_url: url::Url,
    /// Timeout for the http requests sent to avoid potentially permanently hanging requests.
    pub timeout: Duration,
    /// Strategy for function validation strategy. Whenever ChatGPT fails to call a function correctly, this strategy is applied.
    #[cfg(feature = "functions")]
    pub function_validation: FunctionValidationStrategy,
}

impl Default for ModelConfiguration {
    fn default() -> Self {
        Self {
            engine: Default::default(),
            temperature: 0.5,
            top_p: 1.0,
            max_tokens: Some(16),
            presence_penalty: 0.0,
            frequency_penalty: 0.0,
            reply_count: 1,
            api_url: url::Url::from_str("https://api.openai.com/v1/chat/completions").unwrap(),
            timeout: Duration::from_secs(10),
            #[cfg(feature = "functions")]
            function_validation: FunctionValidationStrategy::default(),
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
