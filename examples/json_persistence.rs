use std::env::args;

use chatgpt::prelude::*;

#[cfg(feature = "json")]
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
    conversation
        .send_message("Now could you do the same, but for the Zig language?")
        .await?;

    // Storing the conversation in a local JSON file
    conversation
        .save_history_json("example_conversation.json")
        .await?;
    drop(conversation);

    let mut new = client
        .restore_conversation_json("example_conversation.json")
        .await?;

    let response = new
        .send_message("And can you also do the same for Java?")
        .await?;

    println!("Response for Java: {}", response.message().content());

    Ok(())
}
