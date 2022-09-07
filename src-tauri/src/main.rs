#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use printnanny_imager::disks::get_serializable_disks;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn list_disks(name: &str) -> String {
    get_serializable_disks()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_disks])
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
