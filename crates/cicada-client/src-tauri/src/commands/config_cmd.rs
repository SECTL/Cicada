use crate::autostart;
use cicada_core::config::{load_config, save_config, AppConfig};
use tauri::{Emitter, Manager};

#[tauri::command]
pub fn get_config() -> AppConfig {
    load_config()
}

#[tauri::command]
pub fn update_config(config: AppConfig, app: tauri::AppHandle) -> AppConfig {
    save_config(&config);

    if config.behavior.auto_start {
        let _ = autostart::enable_autostart();
    } else {
        let _ = autostart::disable_autostart();
    }

    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_always_on_top(config.behavior.floating_topmost);
    }

    let _ = app.emit("config-updated", &config);
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
