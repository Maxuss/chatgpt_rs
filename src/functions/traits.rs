use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

// deserialize -> parsing result from API
// schema -> providing description
pub trait FunctionArgument<'de>: Deserialize<'de> + JsonSchema { }

impl<'de, T: Deserialize<'de> + JsonSchema> FunctionArgument<'de> for T { }
