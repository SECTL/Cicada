use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, Runtime,
};

pub fn create_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    let show_i = MenuItem::with_id(app, "show", "显示浮窗", true, None::<&str>)?;
    let hide_i = MenuItem::with_id(app, "hide", "隐藏浮窗", true, None::<&str>)?;
    let settings_i = MenuItem::with_id(app, "settings", "设置...", true, None::<&str>)?;
    let about_i = MenuItem::with_id(app, "about", "关于知了", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    let menu = Menu::with_items(
        app,
        &[
            &show_i,
            &hide_i,
            &PredefinedMenuItem::separator(app)?,
            &settings_i,
            &about_i,
            &PredefinedMenuItem::separator(app)?,
            &quit_i,
        ],
    )?;

    TrayIconBuilder::with_id("tray")
        .tooltip("知了 - 校园公告投屏")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => {
                let _ = app.emit("show-floating", ());
            }
            "hide" => {
                let _ = app.emit("hide-floating", ());
            }
            "settings" => {
                let _ = app.emit("open-settings", ());
            }
            "about" => {
                let _ = app.emit("open-about", ());
            }
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        })
        .build(app)?;

    Ok(())
}
