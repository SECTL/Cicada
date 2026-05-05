use cicada_core::config;

#[derive(serde::Serialize, Clone)]
pub enum AppMode {
    Display,
    Admin,
}

#[tauri::command]
pub fn set_mode(mode: String) -> Result<String, String> {
    if !["display", "admin"].contains(&mode.as_str()) {
        return Err(format!("Invalid mode: {}", mode));
    }
    config::update_config(|c| {
        c.behavior.default_mode = mode.clone();
    });
    Ok(mode)
}

#[tauri::command]
pub fn get_mode() -> String {
    config::load_config().behavior.default_mode
}
