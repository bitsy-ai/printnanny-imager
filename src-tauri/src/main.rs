#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use tauri_plugin_log::{LogTarget, LoggerBuilder};

use printnanny_imager::app;
use printnanny_imager::bootfile;
use printnanny_imager::disk;
use printnanny_imager::disklist;
use printnanny_imager::password;

#[tauri::command]
async fn list_diskdrive_crossplatform() -> String {
    let disks = disklist::list_disks().await.unwrap();
    serde_json::to_string(&disks).unwrap()
}

#[tauri::command]
async fn write_image(image_path: String, disk_path: String, device_id: String) {
    disk::write_image(image_path, disk_path, device_id).unwrap();
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
            password::hash_password,
            password::compare_password,
            bootfile::write_bootfile,
            write_image,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
