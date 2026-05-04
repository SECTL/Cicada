use std::path::PathBuf;

pub fn enable_autostart() -> Result<(), String> {
    let exe_path = std::env::current_exe().map_err(|e| e.to_string())?;
    let exe_str = exe_path.to_string_lossy();

    if cfg!(target_os = "windows") {
        std::process::Command::new("reg")
            .args([
                "add",
                r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run",
                "/v",
                "Cicada",
                "/t",
                "REG_SZ",
                "/d",
                &format!(r#""{}" --silent"#, exe_str),
                "/f",
            ])
            .output()
            .map_err(|e| e.to_string())?;
    } else if cfg!(target_os = "macos") {
        let plist = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key><string>com.cicada.app</string>
    <key>ProgramArguments</key>
    <array><string>{}</string><string>--silent</string></array>
    <key>RunAtLoad</key><true/>
</dict>
</plist>"#,
            exe_str
        );
        let launch_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Library/LaunchAgents");
        std::fs::create_dir_all(&launch_dir).ok();
        std::fs::write(launch_dir.join("com.cicada.app.plist"), plist)
            .map_err(|e| e.to_string())?;
    } else {
        let desktop = format!(
            "[Desktop Entry]\nType=Application\nName=Cicada\nExec=\"{}\" --silent\nX-GNOME-Autostart-enabled=true\n",
            exe_str
        );
        let autostart_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("autostart");
        std::fs::create_dir_all(&autostart_dir).ok();
        std::fs::write(autostart_dir.join("cicada.desktop"), desktop).map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub fn disable_autostart() -> Result<(), String> {
    if cfg!(target_os = "windows") {
        std::process::Command::new("reg")
            .args([
                "delete",
                r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run",
                "/v",
                "Cicada",
                "/f",
            ])
            .output()
            .map_err(|e| e.to_string())?;
    } else if cfg!(target_os = "macos") {
        let plist = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Library/LaunchAgents/com.cicada.app.plist");
        if plist.exists() {
            std::fs::remove_file(plist).ok();
        }
    } else {
        let desktop = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("autostart/cicada.desktop");
        if desktop.exists() {
            std::fs::remove_file(desktop).ok();
        }
    }
    Ok(())
}

pub fn is_autostart_enabled() -> bool {
    if cfg!(target_os = "windows") {
        if let Ok(output) = std::process::Command::new("reg")
            .args([
                "query",
                r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run",
                "/v",
                "Cicada",
            ])
            .output()
        {
            return output.status.success();
        }
    } else if cfg!(target_os = "macos") {
        return dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Library/LaunchAgents/com.cicada.app.plist")
            .exists();
    } else {
        return dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("autostart/cicada.desktop")
            .exists();
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enable_disable_autostart() {
        if is_autostart_enabled() {
            disable_autostart().ok();
        }
        assert!(!is_autostart_enabled());

        if cfg!(target_os = "linux") {
            enable_autostart().ok();
            assert!(is_autostart_enabled());
            disable_autostart().ok();
            assert!(!is_autostart_enabled());
        }
    }
}
