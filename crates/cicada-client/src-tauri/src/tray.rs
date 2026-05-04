use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
};

pub fn create_tray() -> SystemTray {
    let show = CustomMenuItem::new("show".to_string(), "显示浮窗");
    let hide = CustomMenuItem::new("hide".to_string(), "隐藏浮窗");
    let settings = CustomMenuItem::new("settings".to_string(), "设置...");
    let about = CustomMenuItem::new("about".to_string(), "关于知了");
    let quit = CustomMenuItem::new("quit".to_string(), "退出");

    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(hide)
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(settings)
        .add_item(about)
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new()
        .with_menu(tray_menu)
        .with_tooltip("知了 - 校园公告投屏")
}

pub fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick { .. } => {
            if let Some(window) = app.get_window("main") {
                if window.is_visible().unwrap_or(false) {
                    let _ = window.hide();
                } else {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "show" => {
                let _ = app.emit_all("show-floating", ());
            }
            "hide" => {
                let _ = app.emit_all("hide-floating", ());
            }
            "settings" => {
                let _ = app.emit_all("open-settings", ());
            }
            "about" => {
                let _ = app.emit_all("open-about", ());
            }
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
        },
        _ => {}
    }
}
