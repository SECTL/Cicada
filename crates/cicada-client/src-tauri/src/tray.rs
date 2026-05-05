use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    AppHandle, Manager,
};

pub fn build(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let show = MenuItem::with_id(app, "show_main", "显示主窗口", true, None)?;
    let hide = MenuItem::with_id(app, "hide_main", "隐藏到托盘", true, None)?;
    let floating_show = MenuItem::with_id(app, "show_floating", "显示浮窗", true, None)?;
    let floating_hide = MenuItem::with_id(app, "hide_floating", "隐藏浮窗", true, None)?;
    let settings = MenuItem::with_id(app, "open_settings", "设置", true, None)?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None)?;

    let menu = Menu::with_items(
        app,
        &[
            &show,
            &hide,
            &PredefinedMenuItem::separator(app)?,
            &floating_show,
            &floating_hide,
            &PredefinedMenuItem::separator(app)?,
            &settings,
            &PredefinedMenuItem::separator(app)?,
            &quit,
        ],
    )?;

    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .on_menu_event(move |app, event| match event.id().as_ref() {
            "show_main" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "hide_main" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }
            }
            "show_floating" => {
                let _ = app.emit("floating-cmd", "show");
            }
            "hide_floating" => {
                let _ = app.emit("floating-cmd", "hide");
            }
            "open_settings" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                    let _ = window.emit("navigate-to", "settings");
                }
            }
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let tauri::tray::TrayIconEvent::Click { .. } = event {
                if let Some(window) = tray.app_handle().get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(())
}
