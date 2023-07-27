use crate::functions::{CallableAsyncFunction, FunctionArgument};
use schemars::schema_for;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::marker::PhantomData;

/// A descriptor containing information about a ChatGPT function
#[derive(Debug, Clone)]
pub struct FunctionDescriptor<'a, A: FunctionArgument<'a>> {
    /// Contains the name of the function, by which it will be called, e.g. `my_function`
    pub name: &'a str,
    /// Describes what this function does. Description should clearly state what this function does so ChatGPT understands when to call it.
    pub description: &'a str,
    /// Phantom data used for referencing the type of parameter object.
    pub parameters: PhantomData<A>,
}

impl<'a, A: FunctionArgument<'a>> Serialize for FunctionDescriptor<'a, A> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("FunctionDescriptor", 3)?;
        s.serialize_field("name", self.name)?;
        s.serialize_field("description", self.description)?;
        let schema = schema_for!(A);
        s.serialize_field("parameters", &schema)?;
        s.end()
    }
}

/// This struct represents a ChatGPT function.
#[derive(Debug, Clone)]
pub struct GptFunction<'a, A: FunctionArgument<'a>, C: CallableAsyncFunction<A>> {
    /// Descriptor for this function. See [FunctionDescriptor] fields for details
    pub descriptor: FunctionDescriptor<'a, A>,
    /// Phantom data used for referencing the handler for this function. See [CallableAsyncFunction] for details.
    pub callable: PhantomData<C>,
}
