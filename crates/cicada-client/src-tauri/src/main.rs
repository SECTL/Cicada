use cicada_core::auth::oauth::OAuthClient;
use cicada_core::auth::MemoryStorage;
use cicada_core::config::{self, AppConfig};
use commands::auth::AuthState;
use tauri::{Emitter, Manager};

mod autostart;
mod commands;
mod tray;
mod updater;
mod windows;

fn main() {
    let config = config::load_config();

    
    if config.behavior.auto_start && !autostart::is_autostart_enabled() {
        let _ = autostart::enable_autostart();
    }

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
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .manage(auth_state)
        .manage(config.clone())
        .invoke_handler(tauri::generate_handler![
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
        .setup(move |app| {
            let window = app.get_webview_window("main").expect("main window not found");

            let ds = &config.display;
            if ds.window_width > 0 && ds.window_height > 0 {
                let _ = window.set_size(tauri::Size::Logical(tauri::LogicalSize {
                    width: ds.window_width as f64,
                    height: ds.window_height as f64,
                }));
            }

            if ds.window_position_x >= 0 && ds.window_position_y >= 0 {
                let _ = window.set_position(tauri::Position::Logical(tauri::LogicalPosition {
                    x: ds.window_position_x as f64,
                    y: ds.window_position_y as f64,
                }));
            }

            let _ = window.set_always_on_top(config.behavior.floating_topmost);

            if config.behavior.start_minimized {
                let _ = window.hide();
            } else {
                let _ = window.show();
            }

            let default_mode = config.behavior.default_mode.clone();
            if !default_mode.is_empty() {
                let _ = window.emit("mode-changed", &default_mode);
            }

            let _ = tray::build(app.handle())?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Cicada");
}
