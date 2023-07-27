use crate::functions::{CallableAsyncFunction, FunctionArgument};
use schemars::schema_for;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::future::Future;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct FunctionDescriptor<'a, A: FunctionArgument<'a>> {
    pub name: &'a str,
    pub description: &'a str,
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

#[derive(Debug, Clone)]
pub struct GptFunction<'a, A: FunctionArgument<'a>, C: CallableAsyncFunction<A>> {
    pub descriptor: FunctionDescriptor<'a, A>,
    pub callable: PhantomData<C>,
}
