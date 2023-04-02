use std::{env::args, str::FromStr};

use chatgpt::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Getting the API key here
    let key = args().nth(1).unwrap();

    // Creating a new ChatGPT client with extra settings.
    // Note that it might not require an API key depending on proxy
    let client = ChatGPT::new_with_config(
        key,
        ModelConfigurationBuilder::default()
            .api_url(Url::from_str("https://api.pawan.krd/v1/chat/completions").unwrap())
            .temperature(1.0)
            .engine(ChatGPTEngine::Gpt4_32k)
            .build()
            .unwrap(),
    )?;

    // Sending a message and getting the completion
    let response = client
        .send_message("Describe in five words the Rust programming language.")
        .await?;

    println!("Response: {}", response.message().content);

    Ok(())
}
