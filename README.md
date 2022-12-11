# ChatGPT-rs

This is a reverse-engineered wrapper for the OpenAI's ChatGPT model.

## Usage

```rust
use chatgpt::prelude::*;

#[tokio::main]
async fn main() -> chatgpt::Result<()> {
    // Starting client
    let token: String = std::env::var("SESSION_TOKEN").unwrap(); // obtain the session token. More on session tokens later.
    let mut client = ChatGPT::new(token)?;
    client.refresh_token().await?; // it is recommended to refresh token after creating a client
    
    // sending a simple message
    // normal responses take ~10-30 seconds to complete
    let response: String = client.send_message("Write me an HTTP server in Rust using the Axum framework.").await?;

    // in case dynamic updates are important
    // this method allows to receive the message as a stream
    let mut stream = client.send_message_streaming(None, None, "Write me an HTTP server in Rust using the Axum framework.").await?;
    
    while let Some(part) = stream.next().await {
        // a single response part
        println!("Got response part: {part:?}");
    }

    Ok(())
}
```

## Conversations
Conversations are the threads in which ChatGPT can analyze previous messages and chain it's thoughts.

```rust
use chatgpt::prelude::*;

#[tokio::main]
async fn main() -> chatgpt::Result<()> {
    let token: String = std::env::var("SESSION_TOKEN").unwrap(); 
    let mut client = ChatGPT::new(token)?;
    client.refresh_token().await?;
    
    // We create a new empty conversation
    let mut conversation = client.new_conversation();
    let response: String = conversation.send_message(&client, "Write me a simple HTTP server in Rust").await?;

    // Now we can refer to our previous message when talking to ChatGPT
    let response: String = conversation.send_message(&client, "Now can you rewrite in Kotlin using the ktor framework?").await?;

    // Streamed responses are also supported
    let mut stream = conversation.send_message_streaming(&client, "Now can you rewrite it in TypeScript?").await?;

    while let Some(response) = stream.next() {
        // ...
    }

    Ok(())
}
```

Since conversations only hold little data (conversation ID and latest message ID), you can have multiple conversations at the same time!

## Session Tokens
Session tokens allow access to the OpenAI API. You can find them in the Cookie storage of your browser.

### Chromium-based browsers

Do this on the [ChatGPT website](https://chat.openai.com/chat)
1. Ctrl+Shift+I to open dev tools
2. Navigate to the Application tab
3. On the left, choose Storage->Cookies->https://chat.openai.com/chat
4. Get the value of the cookie with name `__Secure-next-auth.session-token`

![Explained in image](./media/token_chromium.png)

### Firefox-based browsers

Do this on the [ChatGPT website](https://chat.openai.com/chat)
1. Ctrl+Shift+I to open dev tools
2. Navigate to the Storage tab
3. On the left choose Cookies->https://chat.openai.com/chat
4. Get the value of the cookie with name `__Secure-next-auth.session-token`

![Explained in image](./media/token_firefox.png)

## Library roadmap

- [x] Refreshing tokens
- [x] Sending message and receiving response
- [x] Receiving response as a stream
- [x] Scoped conversations
- [x] Multiple conversations at the same time