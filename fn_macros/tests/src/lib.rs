#![cfg(test)]

use chatgpt::functions::gpt_function;

#[test]
pub fn test_derive_function_argument() {
    #[gpt_function]
    fn my_test_function(name: String, age: u16) {

    };

}