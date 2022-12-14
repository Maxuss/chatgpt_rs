mod evade;

use std::str::FromStr;

use fantoccini::{wd::Capabilities, ClientBuilder, Locator};
use reqwest::Url;

use super::AuthenticationData;

/// Authenticates using email and password. This uses headless chrome behind the hood, and may be rather slow.
pub async fn simple_auth<E: AsRef<str>, P: AsRef<str>>(
    email: E,
    password: P,
) -> crate::Result<AuthenticationData> {
    let email = email.as_ref();
    let password = password.as_ref();
    let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36";

    let mut capabilities = Capabilities::new();

    capabilities.insert(
        "goog:chromeOptions".to_owned(),
        serde_json::json!({ "args": Vec::<String>::new() }),
    );

    let client = ClientBuilder::rustls()
        .capabilities(capabilities)
        .connect("http://localhost:9515")
        .await?;
    client.set_ua(user_agent).await?;
    evade::evade(&client).await;

    client.goto("https://chat.openai.com/auth/login").await?;

    let element = client
        .wait()
        .for_element(Locator::Css("#__next .btn_main"))
        .await?;

    element.click().await?;

    client
        .wait()
        .for_element(Locator::Css("#username"))
        .await?
        .send_keys(email)
        .await?;
    client
        .wait()
        .for_element(Locator::Css(r#"button[type="submit"]"#))
        .await?
        .click()
        .await?;
    client
        .wait()
        .for_element(Locator::Css("#password"))
        .await?
        .send_keys(password)
        .await?;

    client
        .wait()
        .for_element(Locator::Css(r#"button[type="submit"]"#))
        .await?
        .click()
        .await?;
    client
        .wait()
        .for_url(Url::from_str("https://chat.openai.com/chat").unwrap())
        .await?;

    let mut cookies = client.get_all_cookies().await?.into_iter();

    let cf_token = cookies
        .find(|it| it.name() == "cf_clearance")
        .ok_or_else(|| {
            crate::err::Error::SimpleAuthFailed(
                "Could not find the cloudflare token in cookie storage.".to_owned(),
            )
        })?;
    let cf_token = cf_token.value();

    let access_token = cookies
        .find(|it| it.name() == "__Secure-next-auth.session-token")
        .ok_or_else(|| {
            crate::err::Error::SimpleAuthFailed(
                "Could not find the auth session token in cookie storage.".to_owned(),
            )
        })?;
    let access_token = access_token.value();

    client.close().await?;

    Ok(AuthenticationData {
        cf_token: cf_token.to_owned(),
        user_agent: user_agent.to_owned(),
        access_token: access_token.to_owned(),
    })
}
