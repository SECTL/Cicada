use std::fs;
use std::path::PathBuf;

use super::settings::AppConfig;

fn config_path() -> PathBuf {
    let base = if cfg!(target_os = "windows") {
        dirs::data_dir().unwrap_or_else(|| PathBuf::from("."))
    } else if cfg!(target_os = "macos") {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Library")
            .join("Application Support")
    } else {
        dirs::config_dir().unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")))
    };
    let dir = base.join("Cicada");
    fs::create_dir_all(&dir).ok();
    dir.join("config.toml")
}

pub fn load_config() -> AppConfig {
    let path = config_path();
    if !path.exists() {
        let default_config = AppConfig::default();
        save_config_at(&default_config, &path);
        return default_config;
    }
    match fs::read_to_string(&path) {
        Ok(content) => toml::from_str(&content).unwrap_or_default(),
        Err(_) => AppConfig::default(),
    }
}

pub fn save_config(config: &AppConfig) {
    let path = config_path();
    save_config_at(config, &path);
}

fn save_config_at(config: &AppConfig, path: &PathBuf) {
    if let Ok(content) = toml::to_string_pretty(config) {
        fs::write(path, content).ok();
    }
}

pub fn update_config<F>(updater: F) -> AppConfig
where
    F: FnOnce(&mut AppConfig),
{
    let mut config = load_config();
    updater(&mut config);
    save_config(&config);
    config
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::settings::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.connection.server_url, "http://localhost:3001");
        assert_eq!(config.display.font_size, 24);
        assert!(!config.behavior.auto_start);
        assert_eq!(config.behavior.default_mode, "display");
    }

    #[test]
    fn test_save_and_load() {
        let mut config = AppConfig::default();
        config.display.font_size = 36;
        save_config(&config);

        let loaded = load_config();
        assert_eq!(loaded.display.font_size, 36);

        save_config(&AppConfig::default());
    }

    #[test]
    fn test_update_config() {
        save_config(&AppConfig::default());

        let updated = update_config(|c| {
            c.connection.server_url = "https://api.example.com".to_string();
        });
        assert_eq!(updated.connection.server_url, "https://api.example.com");

        let loaded = load_config();
        assert_eq!(loaded.connection.server_url, "https://api.example.com");

        save_config(&AppConfig::default());
    }

    #[test]
    fn test_display_settings_defaults() {
        let ds = DisplaySettings::default();
        assert_eq!(ds.font_color, "#FFFFFF");
        assert_eq!(ds.bg_opacity, 0.8);
        assert_eq!(ds.window_width, 400);
        assert_eq!(ds.window_height, 300);
    }

    #[test]
    fn test_behavior_settings_defaults() {
        let bs = BehaviorSettings::default();
        assert!(bs.start_minimized);
        assert!(bs.show_floating_on_start);
        assert!(bs.floating_topmost);
    }

    #[test]
    fn test_connection_settings_defaults() {
        let cs = ConnectionSettings::default();
        assert_eq!(cs.heartbeat_interval_secs, 30);
        assert!(cs.client_id.is_empty());
    }
}
