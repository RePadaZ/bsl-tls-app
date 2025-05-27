mod models {
    pub mod error;
}
mod client_module;
mod error;
mod preflight;
mod utils;

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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
