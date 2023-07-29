use crate::functions::{CallableAsyncFunction, FunctionArgument};
use schemars::schema_for;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use std::marker::PhantomData;
use async_trait::async_trait;
use serde_json::Value;

/// A descriptor containing information about a ChatGPT function
#[derive(Debug, Clone)]
pub struct FunctionDescriptor<A: FunctionArgument> {
    /// Contains the name of the function, by which it will be called, e.g. `my_function`
    pub name: &'static str,
    /// Describes what this function does. Description should clearly state what this function does so ChatGPT understands when to call it.
    pub description: &'static str,
    /// Phantom data used for referencing the type of parameter object.
    pub parameters: PhantomData<A>,
}

impl<A: FunctionArgument> Serialize for FunctionDescriptor<A> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("FunctionDescriptor", 3)?;
        s.serialize_field("name", self.name)?;
        s.serialize_field("description", self.description)?;
        let mut schema = schema_for!(A);
        schema.meta_schema = None; // I don't think ChatGPT will need $schema meta information
        s.serialize_field("parameters", &schema)?;

        s.end()
    }
}

/// Trait that indicates a callable GPT Function. Should not be implemented directly, see [GptFunction] instead.
#[async_trait]
pub trait GptFunctionHolder: Send + Sync {
    /// Attempts to invoke this function and returns the result.
    async fn try_invoke(&self, args: &str) -> crate::Result<serde_json::Value>;
}

/// This struct represents a ChatGPT function.
#[derive(Debug, Clone)]
pub struct GptFunction<A: FunctionArgument, C: CallableAsyncFunction<A>> where A: Send + Sync, C: Send + Sync {
    /// Descriptor for this function. See [FunctionDescriptor] fields for details
    pub descriptor: FunctionDescriptor<A>,
    /// Phantom data used for referencing the handler for this function. See [CallableAsyncFunction] for details.
    pub callable: PhantomData<C>,
}

#[async_trait]
impl<A: FunctionArgument + Send + Sync, C: CallableAsyncFunction<A> + Send + Sync> GptFunctionHolder for GptFunction<A, C> {
    async fn try_invoke(&self, args: &str) -> crate::Result<Value> {
        let args_value: A = serde_json::from_str(args).map_err(crate::err::Error::from)?;
        C::invoke(args_value).await
    }
}

/// Determines how ChatGPT will be calling the functions.
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Serialize)]
pub enum FunctionCallingMode {
    /// ChatGPT automatically determines if it should call a function
    Auto,
    /// ChatGPT does not call any functions
    None
}

/// Determines how this client will validate function calls.
#[derive(Debug, Copy, Clone, Default, PartialOrd, PartialEq)]
pub enum FunctionValidationStrategy {
    /// Whenever ChatGPT attempts to call an undefined function, or calls a functions with wrong parameters, sends a `System` message correcting it.
    Strict,
    /// Whenever ChatGPT attempts to call an undefined function, or calls a functions with wrong parameters, ignores the function call. This is default behaviour
    #[default]
    Loose,
}

/// Represents a function call attempted by ChatGPT API
#[derive(Debug, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct FunctionCall {
    /// Name of the function attempted to call
    pub name: String,
    /// Arguments used to call this function, represented by a stringified JSON Object
    pub arguments: String
}

