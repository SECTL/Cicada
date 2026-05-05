use tauri::AppHandle;

pub struct CicadaTray;

impl CicadaTray {
    pub fn new() -> Self {
        Self
    }
    pub fn build(_app: &AppHandle) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self)
    }
}

pub fn handle_tray_event(_app: &AppHandle, _event: &str) {}
