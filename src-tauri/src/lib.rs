mod commands;

use tauri_plugin_fs::FsExt;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::load_config,
            commands::save_config,
            commands::search_itunes,
            commands::subscribe,
            commands::unsubscribe,
            commands::load_subscriptions,
            commands::load_subscription,
            commands::load_podcast_feed,
        ])
        .setup(|app| {
            // allowed the given directory
            let scope = app.fs_scope();
            scope
                .allow_directory("/", true)
                .expect("Directory access is required");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
