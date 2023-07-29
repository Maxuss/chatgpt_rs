#![cfg(test)]
use chatgpt::client::ChatGPT;
use chatgpt::functions::gpt_function;

/// This is some test function
/// * name - Some test parameter 1
/// * age - Some test parameter 2
#[gpt_function]
async fn my_test_function(name: String, age: u16) {
    println!("{name}: {age}")
}

/// Says hello to the user with provided name
///
/// * name - Name of the user
#[gpt_function]
async fn say_hello(name: String) -> bool {
    println!("Hello, {name}");
    true
}

#[test]
pub fn test_derive_function_argument() {
    assert_eq!(
        my_test_function().descriptor.description,
        "This is some test function"
    )
}

#[tokio::test]
pub async fn test_function_sending() -> chatgpt::Result<()> {
    let client = ChatGPT::new(std::env::var("TEST_API_KEY")?)?;
    let mut conv = client.new_conversation();
    conv.add_function(say_hello())?;

    conv.always_send_functions = true;
    let result = conv.send_message("Could you say hello to user named `maxus`?").await?;

    // better tests maybe?
    assert!(!result.message_choices.is_empty());

    Ok(())
}