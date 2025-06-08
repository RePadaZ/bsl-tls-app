use std::fmt::Display;
use tauri::App;
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

/// Универсальная функция для обработки любых ошибок.
/// Принимает все ошибки которые реализованы через Display.
pub fn handle_error<E: Display>(error: &E, app: &mut App) {
    report_the_error(&error.to_string(), app);
}

/// Показывает сообщение об ошибке и завершает приложение.
fn report_the_error(message: &str, app: &mut App) {
    app.dialog()
        .message(message)
        .kind(MessageDialogKind::Error)
        .title("Ошибка")
        .blocking_show();

    std::process::exit(1);
}
