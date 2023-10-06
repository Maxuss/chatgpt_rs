# ChatGPT-rs

This library is an asynchronous Rust wrapper over the OpenAI ChatGPT API.
It supports conversations, message persistence and ChatGPT functions.

## Regarding ChatGPT Functions

The function API (available in `v1.2.0+`) is currently experimental and *may* not work as intended. 
If you encounter any issues or undefined behaviour, please, create an issue in this repository!

## MSRV

The Minimum Supported Rust Version for this library is 1.71.1

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

## Function Calls

ChatGPT-rs supports function calling API. Requires the `functions` feature.

You can define functions with the `gpt_function` attribute macro, like this:

```rust
use chatgpt::prelude::*;

/// Says hello to a user
/// 
/// * user_name - Name of the user to greet
#[gpt_function]
async fn say_hello(user_name: String) {
    println!("Hello, {user_name}!")
}

// ... within your conversation, before sending first message
let mut conversation = client.new_conversation();
// note that you need to call the function when adding it
conversation.add_function(say_hello());
let response = conversation
    .send_message_functions("Could you greet user with name `maxus`?")
    .await?;
// At this point, if function call was issued it was already processed
// and subsequent response was sent
```

As you can see, GPT functions must have a description so the model knows when to call them and what they do. 
In ChatGPT-rs function descriptions are represented as simple rust docs.
Each argument is documented as `* {argument name} - {argument description}`.
Function arguments are processed from JSON, so as long as they implement `schemars::JsonSchema` 
and `serde::Deserialize` they will be parsed correctly.

By default, ChatGPT-rs uses minimal `schemars` features, enable feature `functions_extra` to add support for
`uuid`, `chrono`, `url` and `either`, or define your own structure and derive `schemars::JsonSchema` and `serde::Deserialize`:

```rust
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(JsonSchema, Deserialize)]
struct Args {
    /// Name of the user
    user_name: String,
    /// New age of the user
    user_age: u16
}

/// Wishes happy birthday to the user
/// 
/// * args - Arguments
#[gpt_function]
async fn happy_birthday(args: Args) {
    println!("Hello, {}, You are now {}!", args.user_name, args.user_age);
}
```

Functions can also return any data (as long as it implements `serde::Serialize`) and it will be returned to the model.

```rust
/// Does some heavy computations and returns result
/// 
/// * input - Input data as vector of floats
#[gpt_function]
async fn do_heavy_computation(input: Vec<f64>) -> Vec<f64> {
    let output: Vec<f64> = // ... Do something with the input ...  
    return output;
}
```

By default, functions are only sent to API by calling the `send_message_functions` method. 
If you wish to enable automatic function sending with each message, you can set the `always_send_functions` property within `Conversation` to true.

Current function limitations are:
* They must be async.
* Since they are counted as tokens, you might want to limit function sending and/or their description length.

### Function Call Validation

[As stated in the official ChatGPT documentation](https://platform.openai.com/docs/guides/gpt/function-calling), ChatGPT may hallucinate nonexistent functions
or provide invalid JSON. To mitigate it, ChatGPT-rs provides `FunctionValidationStrategy`. If set to `Strict` within [the client model configuration](https://docs.rs/chatgpt_rs/latest/chatgpt/config/struct.ModelConfiguration.html),
a system message will be sent to the model correcting it whenever it fails to call function correctly.

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
