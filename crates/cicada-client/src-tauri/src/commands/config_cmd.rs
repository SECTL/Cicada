use crate::autostart;
use cicada_core::config::{load_config, save_config, AppConfig};

#[tauri::command]
pub fn get_config() -> AppConfig {
    load_config()
}

#[tauri::command]
pub fn update_config(config: AppConfig) -> AppConfig {
    save_config(&config);
    config
}

#[tauri::command]
pub fn reset_config() -> AppConfig {
    let default = AppConfig::default();
    save_config(&default);
    default
}

#[tauri::command]
pub fn enable_autostart() -> Result<(), String> {
    autostart::enable_autostart()
}

#[tauri::command]
pub fn disable_autostart() -> Result<(), String> {
    autostart::disable_autostart()
}
