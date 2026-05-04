fn main() {
    // Windows 上 tauri-build 有已知 bug (Os { code: 0 })
    // 完全跳过 tauri-build，手动设置必要的 cfg
    println!("cargo:rustc-cfg=desktop");
    println!("cargo:rustc-cfg=dev");
    println!("cargo:rustc-env=TAURI_ENV_TARGET_TRIPLE=x86_64-pc-windows-msvc");

    // 尝试嵌入应用程序清单
    #[cfg(windows)]
    {
        let manifest_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("cicada-client.manifest");
        if manifest_path.exists() {
            println!("cargo:rustc-link-arg=/MANIFEST:EMBED");
            println!("cargo:rustc-link-arg=/MANIFESTINPUT:{}", manifest_path.display());
        }
    }
}
