/// This module contains a simple authentication implementation
#[cfg(feature = "simple-auth")]
pub mod simple;

/// Contains the authentication data, can be received from the different auth methods
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AuthenticationData {
    /// The cloudflare challenge token. Labelled as `cf_clearance` in the cookies.
    pub cf_token: String,
    /// The user agent, that was used to access the authentication page
    pub user_agent: String,
    /// The OpenAI access token. Labelled as `__Secure-next-auth.session-token` in the cookies
    pub access_token: String,
}
