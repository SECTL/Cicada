use cicada_core::auth::{
    generate_code_challenge, generate_code_verifier, oauth::OAuthClient, MemoryStorage,
    TokenStorage,
};
use cicada_core::types::auth::UserInfo;
use serde::Serialize;
use tauri::State;

pub struct AuthState {
    pub client: OAuthClient,
    pub storage: MemoryStorage,
    pub user_info: std::sync::Mutex<Option<UserInfo>>,
}

#[derive(Serialize)]
pub struct LoginUrl {
    pub url: String,
    pub code_verifier: String,
    pub state: String,
}

#[tauri::command]
pub async fn start_login(state: State<'_, AuthState>) -> Result<LoginUrl, String> {
    let code_verifier = generate_code_verifier();
    let code_challenge = generate_code_challenge(&code_verifier);
    let state_str = uuid::Uuid::new_v4().to_string();

    let url = state
        .client
        .build_authorize_url(&code_challenge, &state_str, None);

    Ok(LoginUrl {
        url,
        code_verifier,
        state: state_str,
    })
}

#[tauri::command]
pub async fn complete_login(
    code: String,
    code_verifier: String,
    state: State<'_, AuthState>,
) -> Result<UserInfo, String> {
    let device_uuid = uuid::Uuid::new_v4().to_string();
    let ip = "127.0.0.1";

    let tokens = state
        .client
        .exchange_code_for_token(&code, &code_verifier, &device_uuid, ip)
        .await
        .map_err(|e| e.to_string())?;

    state
        .storage
        .save_tokens(&tokens)
        .map_err(|e| e.to_string())?;

    let user_info = state
        .client
        .get_userinfo(&tokens.access_token)
        .await
        .map_err(|e| e.to_string())?;

    let mut guard = state.user_info.lock().unwrap();
    *guard = Some(user_info.clone());

    Ok(user_info)
}

#[tauri::command]
pub async fn get_user_info(state: State<'_, AuthState>) -> Result<Option<UserInfo>, String> {
    let guard = state.user_info.lock().unwrap();
    Ok(guard.clone())
}

#[tauri::command]
pub async fn logout(state: State<'_, AuthState>) -> Result<(), String> {
    if let Ok(Some(tokens)) = state.storage.load_tokens() {
        let _ = state.client.logout(&tokens.access_token).await;
    }
    state.storage.clear_tokens().map_err(|e| e.to_string())?;

    let mut guard = state.user_info.lock().unwrap();
    *guard = None;

    Ok(())
}
