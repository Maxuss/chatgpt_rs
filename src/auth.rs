/// Contains the authentication data, can be received from the different auth methods
pub struct AuthenticationData {
    pub cf_token: String,
    pub user_agent: String,
    pub access_token: String,
}
