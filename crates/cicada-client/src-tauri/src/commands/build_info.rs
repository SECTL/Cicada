const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(serde::Serialize, Clone)]
pub struct BuildInfo {
    pub version: String,
    pub git_hash: String,
    pub build_date: String,
    pub rustc_version: String,
    pub target: String,
}

#[tauri::command]
pub fn get_build_info() -> BuildInfo {
    BuildInfo {
        version: VERSION.into(),
        git_hash: option_env!("GIT_HASH").unwrap_or("dev").into(),
        build_date: option_env!("BUILD_DATE").unwrap_or("unknown").into(),
        rustc_version: option_env!("RUSTC_VERSION").unwrap_or("unknown").into(),
        target: std::env::consts::ARCH.into(),
    }
}
