use once_cell::sync::OnceCell;
use serde::Serialize;

use tauri::Manager;
use tauri::{AppHandle, Wry};

static APP_INSTANCE: OnceCell<AppHandle<Wry>> = OnceCell::new();

pub static EVENT_IMAGE_WRITE_PROGRESS: &str = "image_write_progress";

pub struct TauriApp {}
impl TauriApp {
    pub fn global() -> &'static AppHandle<Wry> {
        APP_INSTANCE.get().expect("App instance not initialized")
    }

    pub fn set(app: AppHandle<Wry>) {
        APP_INSTANCE.set(app).unwrap();
    }

    pub fn emit<S: Serialize + Clone>(event: &str, payload: S) {
        TauriApp::global().emit_all(event, payload).unwrap();
    }
}
