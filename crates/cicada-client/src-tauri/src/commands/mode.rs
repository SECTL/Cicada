use serde::Serialize;
use tauri::State;

#[derive(Serialize, Clone)]
pub enum AppMode {
    Display,
    Admin,
}

#[tauri::command]
pub fn set_mode(_mode: String, _state: State<'_, AppMode>) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn get_mode() -> String {
    "display".to_string()
}
