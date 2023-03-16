use std::env::args;

use chatgpt::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Creating a client
    let key = args().nth(1).unwrap();
    let client = ChatGPT::new(key)?;

    // Creating a new directed conversation
    let mut conversation: Conversation = client.new_conversation_directed(
        "You are RustGPT, when answering any questions, you always shift the topic of the conversation to the Rust programming language."
    );

    // Sending messages to the conversation
    let response = conversation
        .send_message("Could you briefly tell me about the Zig programming language?")
        .await?;

    println!("Response: {}", response.message().content);

    Ok(())
}
