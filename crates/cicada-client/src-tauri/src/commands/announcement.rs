use crate::commands::auth::AuthState;
use cicada_api::rest::ApiClient;
use cicada_core::auth::TokenStorage;
use cicada_core::config::load_config;
use cicada_core::types::announcement::{Announcement, AnnouncementCreate, AnnouncementType};
use tauri::State;

#[tauri::command]
pub async fn publish_announcement(
    title: String,
    content_html: String,
    announcement_type: String,
    publisher_name: String,
    auth_state: State<'_, AuthState>,
) -> Result<Announcement, String> {
    let title = title.trim().to_string();
    if title.is_empty() {
        return Err("标题不能为空".to_string());
    }

    let publisher_name = publisher_name.trim().to_string();
    if publisher_name.is_empty() {
        return Err("发布者不能为空".to_string());
    }

    let announcement_type = parse_announcement_type(&announcement_type)?;
    let content_html = if content_html.trim().is_empty() {
        format!("<p>{}</p>", escape_html(&title))
    } else {
        content_html
    };

    let config = load_config();
    let client = ApiClient::new(config.connection.server_url.trim_end_matches('/'));
    let payload = AnnouncementCreate {
        title,
        content_html,
        announcement_type,
        publisher_name,
    };
    let token = auth_state
        .storage
        .load_tokens()
        .map_err(|e| e.to_string())?
        .map(|tokens| tokens.access_token)
        .unwrap_or_else(|| "mock-token".to_string());

    client
        .create_announcement(&payload, &token)
        .await
        .map_err(|e| format!("发布失败: {}", e))
}

fn parse_announcement_type(value: &str) -> Result<AnnouncementType, String> {
    match value {
        "normal" => Ok(AnnouncementType::Normal),
        "emergency" => Ok(AnnouncementType::Emergency),
        "notice" => Ok(AnnouncementType::Notice),
        _ => Err(format!("未知公告类型: {}", value)),
    }
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
