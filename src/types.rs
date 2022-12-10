use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SessionRefresh {
    pub user: User,
    pub expires: String,
    #[serde(rename = "accessToken")]
    pub access_token: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub image: String,
    pub picture: String,
    pub groups: Vec<String>,
    pub features: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum PossiblyError<T> {
    Error { error: String },
    Fine(T),
}
