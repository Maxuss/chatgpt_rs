use std::env::args;

use chatgpt::prelude::*;
use futures_util::StreamExt;
use std::io::{stdout, Write};

/// Requires the `streams` crate feature
#[tokio::main]
async fn main() -> Result<()> {
    // Creating a client
    let key = args().nth(1).unwrap();
    let client = ChatGPT::new(key)?;
    let mut conversation = client.new_conversation();

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

    // Getting another streamed response
    let another_stream = conversation
        .send_message_streaming("Now what about Kotlin?")
        .await?;
    another_stream
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

    Ok(())
}
