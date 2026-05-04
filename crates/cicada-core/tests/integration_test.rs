use cicada_core::auth::{
    generate_code_challenge, generate_code_verifier,
    oauth::OAuthClient, MemoryStorage, TokenStorage,
};

#[test]
fn test_full_oauth_flow_pkce() {
    let verifier = generate_code_verifier();
    let challenge = generate_code_challenge(&verifier);
    assert_eq!(generate_code_challenge(&verifier), challenge);
    assert_ne!(verifier, challenge);
    assert!(verifier.len() >= 43);
    assert!(challenge.len() >= 43);
}

#[tokio::test]
async fn test_oauth_client_creation() {
    let client = OAuthClient::new(
        "pf_test",
        "https://appwrite.sectl.cn",
        "http://localhost/callback",
    );
    let url = client.build_authorize_url("challenge_abc", "state_xyz", None);
    assert!(url.contains("pf_test"));
    assert!(url.contains("challenge_abc"));
    assert!(url.contains("state_xyz"));
}

#[test]
fn test_token_storage_flow() {
    use cicada_core::types::auth::AuthToken;

    let storage = MemoryStorage::new();
    let token = AuthToken {
        access_token: "flow_test_access".to_string(),
        refresh_token: "flow_test_refresh".to_string(),
        token_type: "Bearer".to_string(),
        expires_in: 7200,
        user_id: "flow_user".to_string(),
    };

    assert!(storage.load_tokens().unwrap().is_none());

    storage.save_tokens(&token).unwrap();
    let loaded = storage.load_tokens().unwrap().unwrap();
    assert_eq!(loaded.access_token, "flow_test_access");
    assert_eq!(loaded.refresh_token, "flow_test_refresh");

    storage.clear_tokens().unwrap();
    assert!(storage.load_tokens().unwrap().is_none());
}

#[test]
fn test_config_roundtrip() {
    use cicada_core::config::{load_config, save_config, AppConfig};

    let default = AppConfig::default();
    let mut modified = default.clone();
    modified.display.font_size = 48;
    save_config(&modified);

    let loaded = load_config();
    assert_eq!(loaded.display.font_size, 48);

    save_config(&default);
}
