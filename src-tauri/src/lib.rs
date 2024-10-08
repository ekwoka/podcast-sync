use tauri_plugin_fs::FsExt;
pub mod itunes;
pub mod podcast;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
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
