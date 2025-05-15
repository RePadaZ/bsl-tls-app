mod preflight;
mod errors;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn validate_bsl_path() -> Result<(), errors::AppError> {
    preflight::check_bsl_server_path()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, validate_bsl_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}