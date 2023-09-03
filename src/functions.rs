mod traits;
mod types;

pub use traits::*;
pub use types::*;

// used by proc macros
#[doc(hidden)]
pub use async_trait;
#[doc(hidden)]
pub use schemars;
#[doc(hidden)]
pub use serde;
#[doc(hidden)]
pub use serde_json;

pub use gpt_fn_macros::*;

#[cfg(test)]
mod tests {
    use crate::functions::FunctionDescriptor;
    use schemars::JsonSchema;
    use serde::Deserialize;
    use serde_json::json;
    use std::marker::PhantomData;

    #[test]
    pub fn test_descriptor_serialization() {
        #[derive(Deserialize, JsonSchema)]
        #[allow(dead_code)]
        struct TestArguments {
            /// Some example name
            name: String,
            /// Some example age
            age: u16,
        }

        let test_descriptor = FunctionDescriptor {
            name: "test_descriptor",
            description: "Used for testing descriptor serialization",
            parameters: PhantomData::<TestArguments>,
        };

        let value = serde_json::to_value(test_descriptor).unwrap();
        assert_eq!(
            json!({
                "description": "Used for testing descriptor serialization",
                "name": "test_descriptor",
                "parameters": {
                    "properties": {
                        "age": {
                            "description": "Some example age",
                            "format": "uint16",
                            "minimum": 0.0,
                            "type": "integer"
                        },
                        "name": {
                            "description": "Some example name",
                            "type": "string"
                        }
                    },
                    "required": ["age", "name"],
                    "title": "TestArguments",
                    "type": "object"
                }
            }),
            value
        );
    }
}
