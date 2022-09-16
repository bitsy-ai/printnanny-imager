#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use log::warn;
use tauri_plugin_log::{LogTarget, LoggerBuilder};

use printnanny_imager::app;
use printnanny_imager::disk;

#[tauri::command]
async fn list_diskdrive_crossplatform() -> String {
    let disks = disk::list_disks().await.unwrap();
    serde_json::to_string(&disks).unwrap()
}

#[tauri::command]
async fn write_image(image_path: String, disk: String) -> () {
    disk::write_image(image_path, disk).unwrap();
}

fn main() {
    let targets = [LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview];
    tauri::Builder::default()
        .setup(|app| {
            app::TauriApp::set(app.handle());
            Ok(())
        })
        .plugin(LoggerBuilder::new().targets(targets).build())
        .invoke_handler(tauri::generate_handler![
            list_diskdrive_crossplatform,
            write_image
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
