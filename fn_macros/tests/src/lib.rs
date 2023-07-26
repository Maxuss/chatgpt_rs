#![cfg(test)]

use chatgpt::functions::gpt_function;

#[test]
pub fn test_derive_function_argument() {
    /// This is some test function
    /// * name - Some test parameter 1
    /// * age - Some test parameter 2
    #[gpt_function]
    fn my_test_function(name: String, age: u16) {

    }
}