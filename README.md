# ChatGPT-rs-blocking

Fork of [Async chatgpt_rs](https://github.com/Maxuss/chatgpt_rs)
I needed a blocking variation of this without reqwest

## Usage

Here is a simple usage of the API, getting completion for a single message.
You can see more practical examples in the `examples` directory.


```rust
use chatgpt::prelude::*;

async fn main() -> Result<()> {
    // Getting the API key here
    let key = args().nth(1).unwrap();

    /// Creating a new ChatGPT client.
    /// Note that it requires an API key, and uses
    /// tokens from your OpenAI API account balance.
    let client = ChatGPT::new(key)?;

    /// Sending a message and getting the completion
    let response: CompletionResponse = client
        .send_message("Describe in five words the Rust programming language.")
        .unwrap();

    println!("Response: {}", response.message().content);

    Ok(())
}
```