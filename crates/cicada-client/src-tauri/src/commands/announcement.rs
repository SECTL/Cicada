use cicada_core::types::announcement::Announcement;
use tauri::State;
use std::sync::Mutex;

pub struct AnnouncementState {
    pub announcements: Mutex<Vec<Announcement>>,
}

#[tauri::command]
pub async fn publish_announcement(
    _title: String,
    _content_html: String,
    _announcement_type: String,
    _publisher_name: String,
    _state: State<'_, AnnouncementState>,
) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn get_announcements(_state: State<'_, AnnouncementState>) -> Result<Vec<Announcement>, String> {
    let guard = _state.announcements.lock().unwrap();
    Ok(guard.clone())
}
