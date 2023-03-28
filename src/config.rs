use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ModelConfiguration {
    pub engine: ChatGPTEngine,
    pub temperature: f32,
    pub top_p: f32,
    pub presence_penalty: f32,
    pub frequency_penalty: f32,
    pub reply_count: u32,
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
        }
    }
}

impl ModelConfiguration {
    pub fn with_engine(mut self, engine: ChatGPTEngine) -> Self {
        self.engine = engine;
        self
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature;
        self
    }

    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.top_p = top_p;
        self
    }

    pub fn with_presence_penalty(mut self, presence_penalty: f32) -> Self {
        self.presence_penalty = presence_penalty;
        self
    }

    pub fn with_frequency_penalty(mut self, frequency_penalty: f32) -> Self {
        self.frequency_penalty = frequency_penalty;
        self
    }

    pub fn with_reply_count(mut self, reply_count: u32) -> Self {
        self.reply_count = reply_count;
        self
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub enum ChatGPTEngine {
    #[default]
    Gpt35Turbo,
    Gpt35Turbo_0301,
    Gpt4,
    Gpt4_32k,
    Gpt4_0314,
    Gpt4_32k_0314,
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
