use tauri::{Manager, Runtime, WebviewWindowBuilder};

pub struct WindowManager;

impl WindowManager {
    pub fn new() -> Self {
        Self
    }

    pub fn create_floating_window<R: Runtime>(
        &self,
        app: &tauri::AppHandle<R>,
        label: &str,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        topmost: bool,
    ) -> Result<(), tauri::Error> {
        WebviewWindowBuilder::new(app, label, tauri::WebviewUrl::App("floating".into()))
            .position(x, y)
            .inner_size(width, height)
            .decorations(false)
            .skip_taskbar(true)
            .always_on_top(topmost)
            .resizable(true)
            .visible(true)
            .build()?;

        Ok(())
    }

    pub fn destroy_floating_window<R: Runtime>(&self, app: &tauri::AppHandle<R>, label: &str) {
        if let Some(window) = app.get_webview_window(label) {
            let _ = window.close();
        }
    }

    pub fn show_all<R: Runtime>(&self, app: &tauri::AppHandle<R>) {
        for window in app.webview_windows().values() {
            let _ = window.show();
        }
    }

    pub fn hide_all<R: Runtime>(&self, app: &tauri::AppHandle<R>) {
        for window in app.webview_windows().values() {
            let _ = window.hide();
        }
    }

    pub fn set_topmost<R: Runtime>(&self, app: &tauri::AppHandle<R>, label: &str, topmost: bool) {
        if let Some(window) = app.get_webview_window(label) {
            let _ = window.set_always_on_top(topmost);
        }
    }

    pub fn enumerate_screens<R: Runtime>(app: &tauri::AppHandle<R>) -> Vec<tauri::Monitor> {
        app.available_monitors()
            .unwrap_or_default()
            .into_iter()
            .collect()
    }
}
