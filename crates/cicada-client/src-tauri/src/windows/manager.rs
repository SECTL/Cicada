use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

pub struct WindowManager {
    floating_windows: Vec<String>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            floating_windows: Vec::new(),
        }
    }

    pub fn create_floating_window(
        &mut self,
        app: &tauri::AppHandle,
        label: &str,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        topmost: bool,
    ) -> Result<(), tauri::Error> {
        let url = WebviewUrl::App("floating".into());
        WebviewWindowBuilder::new(app, label, url)
            .position(x, y)
            .inner_size(width, height)
            .decorations(false)
            .skip_taskbar(true)
            .always_on_top(topmost)
            .resizable(true)
            .visible(true)
            .build()?;

        self.floating_windows.push(label.to_string());
        Ok(())
    }

    pub fn destroy_floating_window(&mut self, app: &tauri::AppHandle, label: &str) {
        if let Some(window) = app.get_webview_window(label) {
            let _ = window.close();
        }
        self.floating_windows.retain(|l| l != label);
    }

    pub fn show_all(&self, app: &tauri::AppHandle) {
        for label in &self.floating_windows {
            if let Some(window) = app.get_webview_window(label) {
                let _ = window.show();
            }
        }
    }

    pub fn hide_all(&self, app: &tauri::AppHandle) {
        for label in &self.floating_windows {
            if let Some(window) = app.get_webview_window(label) {
                let _ = window.hide();
            }
        }
    }

    pub fn set_topmost(&self, app: &tauri::AppHandle, label: &str, topmost: bool) {
        if let Some(window) = app.get_webview_window(label) {
            let _ = window.set_always_on_top(topmost);
        }
    }

    pub fn enumerate_screens(_app: &tauri::AppHandle) -> Vec<String> {
        vec!["primary".into()]
    }
}
