# ChatGPT-rs

This library has now been rewritten to use official OpenAI's ChatGPT API, instead of other unofficial workarounds.

## ⚠️ Important ⚠️
If you have previously used this library, older versions (0.6.0 and below) are deprecated, unsupported, and *will* produce errors. 
You should use the latest version, which is `1.1.2`.

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

## Streaming Responses

If you wish to gradually build the response message, you may use the `streams` feature (not enabled by default)
of the crate, and special methods to request streamed responses.

Here is an example:

```rust
// Acquiring a streamed response
// Note, that the `futures_util` crate is required for most
// stream related utility methods
let stream = client
    .send_message_streaming("Could you name me a few popular Rust backend server frameworks?")
    .await?;

// Iterating over stream contents
stream
    .for_each(|each| async move {
        match each {
            ResponseChunk::Content {
                delta,
                response_index: _,
            } => {
                // Printing part of response without the newline
                print!("{delta}");
                // Manually flushing the standard output, as `print` macro does not do that
                stdout().lock().flush().unwrap();
            }
            _ => {}
        }
    })
    .await;
}

```

Note that the returned streams normally don't have any utility methods, so you will have to use a `StreamExt` method from your async library of choice (e.g. `futures-util` or `tokio`).

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

### Conversation Streaming

Conversations also support returning streamed responses (with the `streams` feature). 

**NOTE:** Streamed responses *do not* automatically save returned message to history, so you will have to do it manually by yourself.

Here is an example:

```rust
// Acquiring a streamed response
// Note, that the `futures_util` crate is required for most
// stream related utility methods
let mut stream = conversation
    .send_message_streaming("Could you name me a few popular Rust backend server frameworks?")
    .await?;

    // Iterating over a stream and collecting the results into a vector
let mut output: Vec<ResponseChunk> = Vec::new();
while let Some(chunk) = stream.next().await {
    match chunk {
        ResponseChunk::Content {
            delta,
            response_index,
        } => {
            // Printing part of response without the newline
            print!("{delta}");
            // Manually flushing the standard output, as `print` macro does not do that
            stdout().lock().flush().unwrap();
            output.push(ResponseChunk::Content {
                delta,
                response_index,
            });
        }
        // We don't really care about other types, other than parsing them into a ChatMessage later
        other => output.push(other),
    }
}

// Parsing ChatMessage from the response chunks and saving it to the conversation history
let messages = ChatMessage::from_response_chunks(output);
conversation.history.push(messages[0].to_owned());
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

## Advanced configuration

You can configure your model further with `ModelConfigurationBuilder`, which also
allows to use proxies:

```rust
// Getting the API key here
let key = args().nth(1).unwrap();

// Creating a new ChatGPT client with extra settings.
// Note that it might not require an API key depending on proxy
let client = ChatGPT::new_with_config(
    key,
    ModelConfigurationBuilder::default()
        .api_url("https://api.pawan.krd/v1/chat/completions")
        .temperature(1.0)
        .engine(ChatGPTEngine::Gpt4_32k)
        .build()
        .unwrap(),
)?;
```