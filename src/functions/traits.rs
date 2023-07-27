use serde::Deserialize;
use schemars::JsonSchema;

// deserialize -> parsing result from API
// schema -> providing description
pub trait FunctionArgument<'de>: Deserialize<'de> + JsonSchema {}

impl<'de, T: Deserialize<'de> + JsonSchema> FunctionArgument<'de> for T {}

#[async_trait::async_trait]
pub trait CallableAsyncFunction<A> {
    async fn invoke(arguments: A);
}
