use global_hotkey::hotkey::HotKeyParseError;
use std::io;
use tauri_plugin_store::Error as StoreError;
use thiserror::Error;

// Список ошибок приложений
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Не найден BSL Server. Переустановите приложение.")]
    InvalidBslPath,

    #[error("Ошибка получения доступа к папке или файлу: {0}")]
    Io(#[from] io::Error),

    #[error("Ошибка хранилища настроек приложения: {0}")]
    Store(#[from] StoreError),

    #[error("Ошибка парсинга горячей клавиши: {0}")]
    HotKey(#[from] HotKeyParseError),

    #[error("Ошибка чтения данных из настроек")]
    HotkeyNotConfigured,
}
