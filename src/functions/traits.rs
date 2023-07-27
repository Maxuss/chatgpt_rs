use async_trait::async_trait;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};

// deserialize -> parsing result from API
// schema -> providing description
pub trait FunctionArgument<'de>: Deserialize<'de> + JsonSchema {}

impl<'de, T: Deserialize<'de> + JsonSchema> FunctionArgument<'de> for T {}

#[async_trait::async_trait]
pub trait CallableAsyncFunction<A> {
    async fn invoke(arguments: A);
}
