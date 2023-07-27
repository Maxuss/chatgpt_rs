mod traits;
mod types;

pub use traits::*;
pub use types::*;

// used by proc macros
#[doc(hidden)]
pub use serde;
#[doc(hidden)]
pub use schemars as schema;
#[doc(hidden)]
pub use async_trait as async_trait;

pub use gpt_fn_macros::*;

#[cfg(test)]
mod tests {
    use schemars::JsonSchema;
    use serde::Deserialize;
    use serde_json::json;
    use crate::functions::FunctionDescriptor;

    #[test]
    pub fn test_descriptor_serialization() {
        #[derive(Deserialize, JsonSchema)]
        struct TestArguments {
            /// Some example name
            name: String,
            /// Some example age
            age: u16
        }

        let test_descriptor = FunctionDescriptor {
            name: "test_descriptor",
            description: "Used for testing descriptor serialization",
            parameters: TestArguments {
                name: "John Doe".to_string(),
                age: 34,
            },
        };

        let value = serde_json::to_value(test_descriptor).unwrap();
        assert_eq!(json!({
            "description": "Used for testing descriptor serialization",
            "name": "test_descriptor",
            "parameters": {
                "$schema": "http://json-schema.org/draft-07/schema#", // schemars shenanigans
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
        }), value);
    }
}