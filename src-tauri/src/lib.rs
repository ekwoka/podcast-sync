use std::{path::PathBuf, str::FromStr};

use api_types::config::Config;
use tauri::{path::BaseDirectory, Manager};
use tauri_plugin_fs::{FsExt, SafeFilePath};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

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
fn save_config<R: tauri::Runtime>(webview: tauri::Webview<R>, config: Config) {
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
fn load_config<R: tauri::Runtime>(webview: tauri::Webview<R>) -> Config {
    let path = resolve_path(
        &webview,
        SafeFilePath::from_str("config.ron").unwrap(),
        Some(BaseDirectory::AppConfig),
    );
    ron::de::from_reader(std::fs::File::open(path.unwrap()).unwrap()).unwrap()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![load_config, save_config])
        .setup(|app| {
            // allowed the given directory
            let scope = app.fs_scope();
            scope.allow_directory("/", true);
            dbg!(scope.allowed());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
