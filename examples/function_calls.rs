use chatgpt::prelude::*;
use lazy_static::lazy_static;
use serde::Serialize;
use std::env::args;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum FunctionResult {
    Success,
    Failure,
}

// Lazy global state
lazy_static! {
    pub static ref EXISTING_USERS: Vec<String> = vec![
        "maxus".into(),
        "user1".into(),
        "user2".into(),
        "user3".into()
    ];
}

/// Sends message to a certain user. Returns `failure` if user does not exist.
///
/// * user - Name of the user
/// * message - Message to be sent
#[gpt_function]
async fn send_message(user: String, message: String) -> FunctionResult {
    if !EXISTING_USERS.contains(&user) {
        FunctionResult::Failure
    } else {
        println!("Incoming message for {user}: {message}");
        FunctionResult::Success
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Getting the API key here
    let key = args().nth(1).unwrap();

    // Creating a new ChatGPT client and conversation
    let client = ChatGPT::new_with_config(
        key,
        ModelConfigurationBuilder::default()
            .function_validation(FunctionValidationStrategy::Strict)
            .build()
            .unwrap(),
    )?;
    let mut conv = client.new_conversation();

    // Adding the functions
    conv.add_function(send_message())?;

    // Sending message with function
    let response = conv
        .send_message_functions("Could you please send a test message to user `maxus`?")
        .await?;

    println!("Response: {}", response.message().content);

    Ok(())
}
