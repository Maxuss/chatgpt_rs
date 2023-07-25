// deserialize -> parsing result from API
// schema -> providing description
pub trait FunctionArgument<'de>: serde::Deserialize<'de> + schemars::JsonSchema { }

impl<'de, T: serde::Deserialize<'de> + schemars::JsonSchema> FunctionArgument<'de> for T { }