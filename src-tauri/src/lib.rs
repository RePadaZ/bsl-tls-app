mod models {
    pub mod error;
}
mod preflight;
mod utils;

use crate::models::error::AppError;

#[tauri::command]
fn validate_bsl_path() -> Result<(), AppError> {
    preflight::check_bsl_server_path()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            #[cfg(desktop)]
            preflight::check_local_store_data(app).expect("error loading store");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![validate_bsl_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
