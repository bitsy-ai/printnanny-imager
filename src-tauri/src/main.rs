#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri_plugin_log::{LogTarget, LoggerBuilder};

use printnanny_imager::disk;

#[tauri::command]
async fn list_diskdrive_crossplatform() -> String {
    let disks = disk::list_disks().await.unwrap();
    serde_json::to_string(&disks).unwrap()
}

fn main() {
    let targets = [LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview];
    tauri::Builder::default()
        .plugin(LoggerBuilder::new().targets(targets).build())
        .invoke_handler(tauri::generate_handler![list_diskdrive_crossplatform])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
