use headless_chrome::Browser;

use super::AuthenticationData;

/// Authenticates using email and password. This uses headless chrome behind the hood, and may be rather slow.
pub fn simple_auth<E: AsRef<str>, P: AsRef<str>>(
    email: E,
    password: P,
) -> crate::Result<AuthenticationData> {
    let email = email.as_ref();
    let password = password.as_ref();
    let user_agent = "Mozilla/5.0 (Windows NT 10.0; rv:107.0) Gecko/20100101 Firefox/107.0";

    let browser = Browser::default()?;
    let tab = browser.wait_for_initial_tab()?;
    tab.set_user_agent(
        user_agent,
        Some("en-US,en;q=0.9,hi;q=0.8,es;q=0.7,lt;q=0.6"),
        Some("Windows"),
    )?;

    tab.navigate_to("https://chat.openai.com/auth/login")?;

    let element = tab.wait_for_element("#__next .btn-primary")?;

    element.click()?;
    tab.wait_until_navigated()?;

    tab.wait_for_element("#username")?.click()?;
    tab.type_str(email)?;
    tab.find_element(r#"button[type="submit"]"#)?.click()?;
    tab.wait_for_element("#password")?.click()?;
    tab.type_str(password)?;

    tab.find_element(r#"button[type="submit"]"#)?.click()?;
    tab.wait_until_navigated()?;

    let mut cookies = tab.get_cookies()?.into_iter();

    let cf_token = cookies
        .find(|it| it.name == "cf_clearance")
        .ok_or_else(|| {
            crate::err::Error::SimpleAuthFailed(
                "Could not find the cloudflare token in cookie storage.".to_owned(),
            )
        })?
        .value;

    let access_token = cookies
        .find(|it| it.name == "__Secure-next-auth.session-token")
        .ok_or_else(|| {
            crate::err::Error::SimpleAuthFailed(
                "Could not find the auth session token in cookie storage.".to_owned(),
            )
        })?
        .value;

    tab.close(false)?;
    drop(browser);

    Ok(AuthenticationData {
        cf_token,
        user_agent: user_agent.to_owned(),
        access_token,
    })
}
