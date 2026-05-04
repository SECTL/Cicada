use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DisplaySettings {
    pub font_size: u32,
    pub font_color: String,
    pub font_family: String,
    pub bg_color: String,
    pub bg_opacity: f32,
    pub window_width: u32,
    pub window_height: u32,
    pub window_position_x: i32,
    pub window_position_y: i32,
}

impl Default for DisplaySettings {
    fn default() -> Self {
        Self {
            font_size: 24,
            font_color: "#FFFFFF".to_string(),
            font_family: "sans-serif".to_string(),
            bg_color: "#000000".to_string(),
            bg_opacity: 0.8,
            window_width: 400,
            window_height: 300,
            window_position_x: -1,
            window_position_y: -1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BehaviorSettings {
    pub auto_start: bool,
    pub start_minimized: bool,
    pub show_floating_on_start: bool,
    pub floating_topmost: bool,
    pub default_mode: String,
}

impl Default for BehaviorSettings {
    fn default() -> Self {
        Self {
            auto_start: false,
            start_minimized: true,
            show_floating_on_start: true,
            floating_topmost: true,
            default_mode: "display".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectionSettings {
    pub server_url: String,
    pub ws_url: String,
    pub client_id: String,
    pub heartbeat_interval_secs: u64,
}

impl Default for ConnectionSettings {
    fn default() -> Self {
        Self {
            server_url: "http://localhost:3001".to_string(),
            ws_url: "ws://localhost:3001/ws/v1/announcements".to_string(),
            client_id: String::new(),
            heartbeat_interval_secs: 30,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    pub connection: ConnectionSettings,
    pub display: DisplaySettings,
    pub behavior: BehaviorSettings,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            connection: ConnectionSettings::default(),
            display: DisplaySettings::default(),
            behavior: BehaviorSettings::default(),
        }
    }
}
