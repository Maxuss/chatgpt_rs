// #![cfg(test)]

use std::future::Future;
use std::marker::PhantomData;
use chatgpt::functions::{FunctionDescriptor, gpt_function, GptFunction};
use chatgpt::functions::schema::JsonSchema;
use chatgpt::functions::serde::Deserialize;


/// This is some test function
/// * name - Some test parameter 1
/// * age - Some test parameter 2
#[gpt_function]
pub async fn my_test_function(name: String, age: u16) {
    println!("{name}: {age}")
}


#[test]
pub fn test_derive_function_argument() {
    assert_eq!(
        my_test_function().descriptor.description,
        "This is some test function"
    )
}

