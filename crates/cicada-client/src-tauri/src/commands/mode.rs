use cicada_core::config;
use tauri::{Emitter, Manager};

#[derive(serde::Serialize, Clone)]
pub enum AppMode {
    Display,
    Admin,
}

#[tauri::command]
pub fn set_mode(mode: String, app: tauri::AppHandle) -> Result<String, String> {
    let valid_modes = ["display", "admin"];
    if !valid_modes.contains(&mode.as_str()) {
        return Err(format!("Invalid mode: {}. Valid: display, admin", mode));
    }

    config::update_config(|c| {
        c.behavior.default_mode = mode.clone();
    });

    let _ = app.emit("mode-changed", &mode);
    Ok(mode)
}

#[tauri::command]
pub fn get_mode() -> String {
    let config = config::load_config();
    config.behavior.default_mode
}
