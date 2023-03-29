use std::env::args;

use chatgpt::prelude::*;
use futures_util::StreamExt;
use std::io::{stdout, Write};

#[cfg(feature = "streams")]
#[tokio::main]
async fn main() -> Result<()> {
    // Creating a client

    use futures::Stream;
    let key = args().nth(1).unwrap();
    let client = ChatGPT::new(key)?;

    // Acquiring a streamed response
    // Note, that the `futures` dependency is required
    // to manage streams, and `futures_util` for most utility methods
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
    Ok(())
}
