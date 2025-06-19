mod models {
    pub mod custom_type;
    pub mod error;
    pub mod standard_setting;
}
mod client_module;
mod error;
mod preflight;
mod utils;

use crate::models::custom_type::Settings;
use crate::models::error::AppError;
use std::collections::HashMap;

#[tauri::command]
fn get_data_setting(app: tauri::AppHandle) -> Result<HashMap<String, String>, AppError> {
    client_module::get_data_setting(app)
}

#[tauri::command]
fn save_data_setting(app: tauri::AppHandle, setting: Settings) -> Result<(), AppError> {
    client_module::save_data_setting(app, setting)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // Проверяем приложение перед самой загрузкой
            #[cfg(desktop)]
            if let Err(e) = preflight::check_app_for_start(app) {
                error::handle_error(&e, app);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_data_setting,
            save_data_setting
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
