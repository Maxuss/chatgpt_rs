use std::env::args;

use chatgpt::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Creating a client
    let key = args().nth(1).unwrap();
    let client = ChatGPT::new(key)?;

    // Creating a new conversation
    let mut conversation = client.new_conversation();

    // Sending messages to the conversation
    conversation
        .send_message("Could you describe the Rust programming language in 5 words?")
        .await?;
    let response = conversation
        .send_message("Now could you do the same, but for Kotlin?")
        .await?;
    println!("Response for Kotlin: {}", response.message().content());

    // The history is preserved and is sent to the API each call
    for message in &conversation.history {
        println!("Message in the history: {message:#?}")
    }

    Ok(())
}
