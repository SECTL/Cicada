use reqwest::Client;
use thiserror::Error;

use cicada_core::types::announcement::{
    Announcement, AnnouncementCreate, AnnouncementListResponse, AnnouncementType,
};

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("unauthorized")]
    Unauthorized,

    #[error("not found")]
    NotFound,

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("server error: {0}")]
    ServerError(u16),

    #[error("parse error: {0}")]
    ParseError(String),
}

pub struct ApiClient {
    base_url: String,
    http_client: Client,
}

impl ApiClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            http_client: Client::new(),
        }
    }

    pub async fn get_announcements(
        &self,
        page: Option<u32>,
        per_page: Option<u32>,
        type_filter: Option<AnnouncementType>,
    ) -> Result<AnnouncementListResponse, ApiError> {
        let mut url = format!("{}/api/v1/announcements", self.base_url);
        let mut params = Vec::new();

        if let Some(p) = page {
            params.push(format!("page={}", p));
        }
        if let Some(pp) = per_page {
            params.push(format!("per_page={}", pp));
        }
        if let Some(t) = type_filter {
            params.push(format!("type={}", t));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let resp = self.http_client.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    pub async fn get_announcement(&self, id: &str) -> Result<Announcement, ApiError> {
        let url = format!("{}/api/v1/announcements/{}", self.base_url, id);
        let resp = self.http_client.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    pub async fn create_announcement(
        &self,
        payload: &AnnouncementCreate,
        token: &str,
    ) -> Result<Announcement, ApiError> {
        let url = format!("{}/api/v1/announcements", self.base_url);
        let resp = self
            .http_client
            .post(&url)
            .bearer_auth(token)
            .json(payload)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    pub async fn delete_announcement(&self, id: &str, token: &str) -> Result<(), ApiError> {
        let url = format!("{}/api/v1/announcements/{}", self.base_url, id);
        let resp = self
            .http_client
            .delete(&url)
            .bearer_auth(token)
            .send()
            .await?;

        match resp.status().as_u16() {
            204 | 200 => Ok(()),
            401 => Err(ApiError::Unauthorized),
            404 => Err(ApiError::NotFound),
            s => Err(ApiError::ServerError(s)),
        }
    }

    async fn handle_response<T: serde::de::DeserializeOwned>(
        resp: reqwest::Response,
    ) -> Result<T, ApiError> {
        let status = resp.status().as_u16();
        match status {
            200 | 201 => {
                let body = resp
                    .text()
                    .await
                    .map_err(|e| ApiError::ParseError(e.to_string()))?;
                serde_json::from_str(&body).map_err(|e| ApiError::ParseError(e.to_string()))
            }
            401 => Err(ApiError::Unauthorized),
            404 => Err(ApiError::NotFound),
            400 => {
                let body = resp.text().await.unwrap_or_default();
                Err(ApiError::BadRequest(body))
            }
            s => Err(ApiError::ServerError(s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cicada_core::types::announcement::*;
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_announcements() {
        let server = MockServer::start().await;
        let client = ApiClient::new(server.uri());

        Mock::given(method("GET"))
            .and(path("/api/v1/announcements"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "announcements": [{
                    "id": "550e8400-e29b-41d4-a716-446655440000",
                    "title": "test",
                    "content_html": "<p>test</p>",
                    "announcement_type": "normal",
                    "publisher_name": "Tester",
                    "publisher_user_id": "user_001",
                    "created_at": "2026-06-15T08:00:00Z",
                    "updated_at": null
                }],
                "total": 1,
                "page": 1,
                "per_page": 20
            })))
            .mount(&server)
            .await;

        let result = client.get_announcements(None, None, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_announcement() {
        let server = MockServer::start().await;
        let client = ApiClient::new(server.uri());

        Mock::given(method("POST"))
            .and(path("/api/v1/announcements"))
            .respond_with(ResponseTemplate::new(201).set_body_json(json!({
                "id": "550e8400-e29b-41d4-a716-446655440001",
                "title": "new",
                "content_html": "<p>new</p>",
                "announcement_type": "normal",
                "publisher_name": "Teacher",
                "publisher_user_id": "user_002",
                "created_at": "2026-06-15T09:00:00Z",
                "updated_at": null
            })))
            .mount(&server)
            .await;

        let payload = AnnouncementCreate {
            title: "new".to_string(),
            content_html: "<p>new</p>".to_string(),
            announcement_type: AnnouncementType::Normal,
            publisher_name: "Teacher".to_string(),
        };

        let result = client.create_announcement(&payload, "token").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_error_unauthorized() {
        let server = MockServer::start().await;
        let client = ApiClient::new(server.uri());

        Mock::given(method("GET"))
            .and(path("/api/v1/announcements"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let result = client.get_announcements(None, None, None).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }
}
