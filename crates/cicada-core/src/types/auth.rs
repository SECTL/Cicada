use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthToken {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserInfo {
    pub user_id: String,
    pub user_name: String,
    pub email: String,
    pub permission: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TokenStatus {
    pub active: bool,
    pub user_id: String,
    pub client_id: String,
    pub expires_at: Option<String>,
    pub scope: Option<String>,
}
