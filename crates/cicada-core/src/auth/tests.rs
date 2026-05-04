#[cfg(test)]
mod tests {
    use super::super::oauth::*;
    use crate::auth::error::AuthError;
    use crate::types::auth::{AuthToken, TokenStatus, UserInfo};
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn test_pkce_generation() {
        let verifier = generate_code_verifier();
        assert!(!verifier.is_empty(), "verifier should not be empty");

        let challenge = generate_code_challenge(&verifier);
        assert!(!challenge.is_empty(), "challenge should not be empty");

        let challenge2 = generate_code_challenge(&verifier);
        assert_eq!(
            challenge, challenge2,
            "same verifier should produce same challenge"
        );
    }

    #[test]
    fn test_authorize_url_building() {
        let client = OAuthClient::new(
            "pf_test123",
            "https://appwrite.sectl.cn",
            "http://localhost:9999/callback",
        );
        let url = client.build_authorize_url("challenge_abc", "state_xyz", Some("user:read"));
        assert!(url.contains("client_id=pf_test123"));
        assert!(url.contains("redirect_uri=http://localhost:9999/callback"));
        assert!(url.contains("response_type=code"));
        assert!(url.contains("code_challenge=challenge_abc"));
        assert!(url.contains("code_challenge_method=S256"));
        assert!(url.contains("state=state_xyz"));
        assert!(url.contains("scope=user:read"));
    }

    #[tokio::test]
    async fn test_token_exchange_success() {
        let mock_server = MockServer::start().await;
        let client = OAuthClient::new("pf_test", mock_server.uri(), "http://localhost/callback");

        Mock::given(method("POST"))
            .and(path("/api/oauth/token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "access_token": "access_abc",
                "refresh_token": "refresh_abc",
                "token_type": "Bearer",
                "expires_in": 3600,
                "user_id": "user_xyz"
            })))
            .mount(&mock_server)
            .await;

        let result = client
            .exchange_code_for_token("auth_code", "verifier", "device-uuid-123", "127.0.0.1")
            .await;

        assert!(result.is_ok());
        let token = result.unwrap();
        assert_eq!(token.access_token, "access_abc");
        assert_eq!(token.refresh_token, "refresh_abc");
        assert_eq!(token.user_id, "user_xyz");
    }

    #[tokio::test]
    async fn test_token_refresh() {
        let mock_server = MockServer::start().await;
        let client = OAuthClient::new("pf_test", mock_server.uri(), "http://localhost/callback");

        Mock::given(method("POST"))
            .and(path("/api/oauth/refresh"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "access_token": "new_access",
                "refresh_token": "new_refresh",
                "token_type": "Bearer",
                "expires_in": 3600,
                "user_id": "user_xyz"
            })))
            .mount(&mock_server)
            .await;

        let result = client
            .refresh_access_token("old_refresh", "device-123", "127.0.0.1")
            .await;
        assert!(result.is_ok());
        let token = result.unwrap();
        assert_eq!(token.access_token, "new_access");
        assert_eq!(token.refresh_token, "new_refresh");
    }

    #[tokio::test]
    async fn test_get_userinfo() {
        let mock_server = MockServer::start().await;
        let client = OAuthClient::new("pf_test", mock_server.uri(), "http://localhost/callback");

        Mock::given(method("GET"))
            .and(path("/api/oauth/userinfo"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "user_id": "user_abc",
                "user_name": "张三",
                "email": "test@example.com",
                "permission": 10
            })))
            .mount(&mock_server)
            .await;

        let result = client.get_userinfo("access_token_xxx").await;
        assert!(result.is_ok());
        let info = result.unwrap();
        assert_eq!(info.user_name, "张三");
        assert_eq!(info.permission, 10);
    }

    #[tokio::test]
    async fn test_introspect_token() {
        let mock_server = MockServer::start().await;
        let client = OAuthClient::new("pf_test", mock_server.uri(), "http://localhost/callback");

        Mock::given(method("POST"))
            .and(path("/api/oauth/introspect"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "active": true,
                "user_id": "user_abc",
                "client_id": "pf_test",
                "expires_at": "2026-01-01T00:00:00Z",
                "scope": "user:read"
            })))
            .mount(&mock_server)
            .await;

        let result = client.introspect_token("access_token_xxx").await;
        assert!(result.is_ok());
        let status = result.unwrap();
        assert!(status.active);
        assert_eq!(status.client_id, "pf_test");
    }

    #[tokio::test]
    async fn test_error_handling_unauthorized() {
        let mock_server = MockServer::start().await;
        let client = OAuthClient::new("pf_test", mock_server.uri(), "http://localhost/callback");

        Mock::given(method("POST"))
            .and(path("/api/oauth/token"))
            .respond_with(ResponseTemplate::new(401).set_body_json(json!({
                "error": "invalid_grant"
            })))
            .mount(&mock_server)
            .await;

        let result = client
            .exchange_code_for_token("bad_code", "verifier", "device-123", "127.0.0.1")
            .await;

        assert!(matches!(result, Err(AuthError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_logout() {
        let mock_server = MockServer::start().await;
        let client = OAuthClient::new("pf_test", mock_server.uri(), "http://localhost/callback");

        Mock::given(method("POST"))
            .and(path("/api/oauth/logout"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "success": true,
                "message": "Token has been revoked"
            })))
            .mount(&mock_server)
            .await;

        let result = client.logout("access_token").await;
        assert!(result.is_ok());
    }
}
