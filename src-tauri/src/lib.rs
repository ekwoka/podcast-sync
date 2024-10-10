use api_types::config::Config;
use tauri_plugin_fs::FsExt;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn load_config() -> Config {
    dbg!("load_config");
    Config {
        latest_message: "Hello, world!".to_string(),
    }
}

#[tauri::command]
fn save_config(config: Config) {
    dbg!(config.to_string().unwrap());
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
