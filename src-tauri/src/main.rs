#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use printnanny_imager::disks::get_serializable_disks;

#[tauri::command]
fn list_disks() -> String {
    get_serializable_disks()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_disks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
