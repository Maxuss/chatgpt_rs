/// Contains the authentication data, can be received from the different auth methods
pub struct AuthenticationData {
    pub(crate) cf_token: String,
    pub(crate) user_agent: String,
    pub(crate) access_token: String,
}
