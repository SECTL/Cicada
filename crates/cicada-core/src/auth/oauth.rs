use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::RngCore;
use reqwest::Client;
use sha2::{Digest, Sha256};

use crate::types::auth::{AuthToken, TokenStatus, UserInfo};

use super::error::AuthError;

pub struct OAuthClient {
    client_id: String,
    base_url: String,
    redirect_uri: String,
    http_client: Client,
}

impl OAuthClient {
    pub fn new(client_id: impl Into<String>, base_url: impl Into<String>, redirect_uri: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
            base_url: base_url.into(),
            redirect_uri: redirect_uri.into(),
            http_client: Client::new(),
        }
    }

    pub fn build_authorize_url(&self, code_challenge: &str, state: &str, scope: Option<&str>) -> String {
        let mut url = format!(
            "{}/oauth/authorize?client_id={}&redirect_uri={}&response_type=code&code_challenge={}&code_challenge_method=S256&state={}",
            self.base_url, self.client_id, self.redirect_uri, code_challenge, state
        );
        if let Some(s) = scope {
            url.push_str(&format!("&scope={}", s));
        }
        url
    }

    pub async fn exchange_code_for_token(
        &self,
        code: &str,
        code_verifier: &str,
        device_uuid: &str,
        ip_address: &str,
    ) -> Result<AuthToken, AuthError> {
        let resp = self
            .http_client
            .post(format!("{}/api/oauth/token", self.base_url))
            .json(&serde_json::json!({
                "grant_type": "authorization_code",
                "code": code,
                "client_id": self.client_id,
                "redirect_uri": self.redirect_uri,
                "code_verifier": code_verifier,
                "device_uuid": device_uuid,
                "ip_address": ip_address,
            }))
            .send()
            .await?;

        if resp.status().is_success() {
            let token: AuthToken = resp.json().await.map_err(|e| AuthError::InvalidResponse(e.to_string()))?;
            Ok(token)
        } else if resp.status().as_u16() == 401 {
            Err(AuthError::Unauthorized)
        } else {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            Err(AuthError::ServerError(format!("HTTP {}: {}", status, body)))
        }
    }

    pub async fn refresh_access_token(
        &self,
        refresh_token: &str,
        device_uuid: &str,
        ip_address: &str,
    ) -> Result<AuthToken, AuthError> {
        let resp = self
            .http_client
            .post(format!("{}/api/oauth/refresh", self.base_url))
            .json(&serde_json::json!({
                "grant_type": "refresh_token",
                "refresh_token": refresh_token,
                "client_id": self.client_id,
                "device_uuid": device_uuid,
                "ip_address": ip_address,
            }))
            .send()
            .await?;

        if resp.status().is_success() {
            let token: AuthToken = resp.json().await.map_err(|e| AuthError::InvalidResponse(e.to_string()))?;
            Ok(token)
        } else if resp.status().as_u16() == 401 {
            Err(AuthError::Unauthorized)
        } else {
            Err(AuthError::ServerError(format!("HTTP {}", resp.status())))
        }
    }

    pub async fn get_userinfo(&self, access_token: &str) -> Result<UserInfo, AuthError> {
        let resp = self
            .http_client
            .get(format!("{}/api/oauth/userinfo", self.base_url))
            .bearer_auth(access_token)
            .send()
            .await?;

        if resp.status().is_success() {
            let info: UserInfo = resp.json().await.map_err(|e| AuthError::InvalidResponse(e.to_string()))?;
            Ok(info)
        } else if resp.status().as_u16() == 401 {
            Err(AuthError::Unauthorized)
        } else {
            Err(AuthError::ServerError(format!("HTTP {}", resp.status())))
        }
    }

    pub async fn introspect_token(&self, access_token: &str) -> Result<TokenStatus, AuthError> {
        let resp = self
            .http_client
            .post(format!("{}/api/oauth/introspect", self.base_url))
            .json(&serde_json::json!({
                "token": access_token,
                "client_id": self.client_id,
            }))
            .send()
            .await?;

        if resp.status().is_success() {
            let status: TokenStatus = resp.json().await.map_err(|e| AuthError::InvalidResponse(e.to_string()))?;
            Ok(status)
        } else {
            Err(AuthError::ServerError(format!("HTTP {}", resp.status())))
        }
    }

    pub async fn logout(&self, access_token: &str) -> Result<(), AuthError> {
        let resp = self
            .http_client
            .post(format!("{}/api/oauth/logout", self.base_url))
            .bearer_auth(access_token)
            .send()
            .await?;

        if resp.status().is_success() || resp.status().as_u16() == 401 {
            Ok(())
        } else {
            Err(AuthError::ServerError(format!("HTTP {}", resp.status())))
        }
    }
}

pub fn generate_code_verifier() -> String {
    let mut random_bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut random_bytes);
    URL_SAFE_NO_PAD.encode(random_bytes)
}

pub fn generate_code_challenge(verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let hash = hasher.finalize();
    URL_SAFE_NO_PAD.encode(hash)
}
