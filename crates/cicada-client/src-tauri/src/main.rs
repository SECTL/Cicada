#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cicada_core::auth::oauth::OAuthClient;
use cicada_core::auth::MemoryStorage;
use cicada_core::config;
use commands::auth::AuthState;
use tauri::Manager;

mod autostart;
mod commands;
mod tray;
mod updater;
mod windows;

fn main() {
    let config = config::load_config();

    let oauth_client = OAuthClient::new(
        config.connection.client_id.clone(),
        "https://appwrite.sectl.cn".to_string(),
        "http://localhost:5173/callback".to_string(),
    );

    let auth_state = AuthState {
        client: oauth_client,
        storage: MemoryStorage::new(),
        user_info: std::sync::Mutex::new(None),
    };

    tauri::Builder::default()
        .manage(auth_state)
        .invoke_handler(tauri::generate_handler![
            commands::announcement::publish_announcement,
            commands::auth::start_login,
            commands::auth::complete_login,
            commands::auth::get_user_info,
            commands::auth::logout,
            commands::config_cmd::get_config,
            commands::config_cmd::update_config,
            commands::config_cmd::reset_config,
            commands::config_cmd::enable_autostart,
            commands::config_cmd::disable_autostart,
            commands::mode::set_mode,
            commands::mode::get_mode,
            commands::build_info::get_build_info,
        ])
        .setup(|app| {
            let window = app.get_window("main").expect("main window not found");
            window.show()?;
            window.set_focus()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Cicada");
}
