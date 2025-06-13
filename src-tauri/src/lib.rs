mod models {
    pub mod error;
    pub mod error_client_module;
    pub mod standard_setting;
}
mod client_module;
mod error;
mod preflight;
mod utils;

use crate::models::error_client_module::ErrorClientModule;
use std::collections::HashMap;

#[tauri::command]
fn all_data_settings(app: tauri::AppHandle) -> Result<HashMap<String, String>, ErrorClientModule> {
    client_module::get_data_settings(app)
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
        .invoke_handler(tauri::generate_handler![all_data_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
