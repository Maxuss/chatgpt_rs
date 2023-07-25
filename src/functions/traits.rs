// deserialize -> parsing result from API
// schema -> providing description
pub trait FunctionArgument: serde::Deserialize + schemars::JsonSchema { }

impl<T: serde::Deserialize + schemars::JsonSchema> FunctionArgument for T { }