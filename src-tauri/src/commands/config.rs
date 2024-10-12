use std::{path::PathBuf, str::FromStr};

use api_types::config::Config;
use tauri::{path::BaseDirectory, Manager};
use tauri_plugin_fs::SafeFilePath;

pub fn resolve_path<R: tauri::Runtime>(
    webview: &tauri::Webview<R>,
    path: SafeFilePath,
    base_dir: Option<BaseDirectory>,
) -> Option<PathBuf> {
    let path = path.into_path().ok()?;
    if let Some(base_dir) = base_dir {
        webview.path().resolve(&path, base_dir).ok()
    } else {
        Some(path)
    }
}

#[tauri::command]
pub fn save_config<R: tauri::Runtime>(webview: tauri::Webview<R>, config: Config) {
    let path = resolve_path(
        &webview,
        SafeFilePath::from_str("config.ron").unwrap(),
        Some(BaseDirectory::AppConfig),
    );
    config
        .to_writer(&mut std::fs::File::create(path.unwrap()).unwrap())
        .unwrap();
}

#[tauri::command]
pub fn load_config<R: tauri::Runtime>(webview: tauri::Webview<R>) -> Config {
    let path = resolve_path(
        &webview,
        SafeFilePath::from_str("config.ron").unwrap(),
        Some(BaseDirectory::AppConfig),
    );
    ron::de::from_reader(std::fs::File::open(path.unwrap()).unwrap()).unwrap()
}
