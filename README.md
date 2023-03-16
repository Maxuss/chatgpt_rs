# Regarding API changes from December 11th 2022
OpenAI made a change to API, and now requires a cloudflare clearance token. Due to this, authentication is becoming complicated. You can view recent updates regarding authentication methods in the [#3 Pull Request](https://github.com/Maxuss/chatgpt_rs/pull/3). The master branch version (and cargo published crate) does not work because of this.

# ChatGPT-rs

This library has now been rewritten to use official OpenAI's ChatGPT API, instead of other unofficial workarounds.

## Usage

Here is a simple usage of the API, getting completion for a single message.
You can see more practical examples in the `examples` directory.


```rust
use chatgpt::prelude::*;

#[tokio::main]
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
        .await?;

    println!("Response: {}", response.message().content);

    Ok(())
}
```

## Conversations

Conversations are the threads in which ChatGPT can analyze previous messages and chain it's thoughts. 
They also automatically store all the message history.

Here is an example:

```rust
// Creating a new conversation
let mut conversation: Conversation = client.new_conversation();

// Sending messages to the conversation
let response_a: CompletionResponse = conversation
    .send_message("Could you describe the Rust programming language in 5 words?")
    .await?;
let response_b: CompletionResponse = conversation
    .send_message("Now could you do the same, but for Kotlin?")
    .await?;

// You can also access the message history itself
for message in &conversation.history {
    println!("{message:#?}")
}
```

This way of creating a conversation creates it with the default introductory message, which roughly is:
`You are ChatGPT, an AI model developed by OpenAI. Answer as concisely as possible. Today is: {today's date}`.

However, you can specify the introductory message yourself this way:

```rust
let mut conversation: Conversation = client.new_conversation_directed("You are RustGPT, when answering any questions, you always shift the topic of the conversation to the Rust programming language.");
// Continue with the new conversation
```

## Conversation Persistence

You can currently store the conversation's message in two formats: JSON or [postcard](https://github.com/jamesmunns/postcard).
They can be toggled on or off using the `json` and `postcard` features respectively.

Since the `ChatMessage` struct derives serde's `Serialize` and `Deserialize` traits, you can also use any serde-compatible serialization library,
as the `history` field and the `Conversation::new_with_history()` method are public in the `Conversation` struct.


### Persistence with JSON
Requires the `json` feature (enabled by default)

```rust
// Create a new conversation here
let mut conversation: Conversation = ...;

// ... send messages to the conversation ...

// Saving the conversation
conversation.save_history_json("my-conversation.json").await?;

// You can later read this conversation history again
let mut restored = client
    .restore_conversation_json("my-conversation.json")
    .await?;
```

### Persistence with Postcard
Requires the `postcard` feature (disabled by default)

```rust
// Create a new conversation here
let mut conversation: Conversation = ...;

// ... send messages to the conversation ...

// Saving the conversation
conversation.save_history_postcard("my-conversation.bin").await?;

// You can later read this conversation history again
let mut restored = client
    .restore_conversation_postcard("my-conversation.bin")
    .await?;
```