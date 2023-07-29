use schemars::JsonSchema;
use serde::de::DeserializeOwned;

// deserialize -> parsing result from API
// schema -> providing description
/// This trait represents an object containing ChatGPT function arguments.
/// To use this trait, just derive [JsonSchema] and [Deserialize]
pub trait FunctionArgument: DeserializeOwned + JsonSchema + Sized {}

impl<T: DeserializeOwned + JsonSchema> FunctionArgument for T {}

/// This trait represents a struct containing actual ChatGPT function handling logic
#[async_trait::async_trait]
pub trait CallableAsyncFunction<A> {
    /// Invokes this function. This method should not be called outside of internal logic.
    async fn invoke(arguments: A) -> crate::Result<serde_json::Value>;
}
