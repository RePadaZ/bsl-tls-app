use crate::models::error::AppError;
use crate::utils::{create_new_store, read_store_data};
use std::env;
use tauri::App;
use tauri_plugin_store::StoreExt;

/// Функция для проверки приложения перед запуском
/// Сюда можно добавлять новые функция перед самим запуском
/// Которые будут отображаться у пользователя в случаи ошибки приложения
pub fn check_app_for_start(app: &mut App) -> Result<(), AppError> {
    check_bsl_server_path()?;
    check_local_store_data(app)?;
    Ok(())
}

/// Проверяет, установлен ли BSL Server в текущей директории
pub fn check_bsl_server_path() -> Result<(), AppError> {
    // Пытаемся получить текущую директорию
    let mut path = env::current_dir()?;

    // Добавляем путь до исполняемого файла
    path.push("bsl-language-server");
    path.push("bsl-language-server.exe");

    // Проверяем, существует ли файл
    if !path.is_file() {
        return Err(AppError::InvalidBslPath);
    }

    Ok(())
}

/// Функция для хранения настроек пользователя.
/// Создает или читает уже существующие настройки приложения.
/// Так же делает первичную инициализацию настроек.
pub fn check_local_store_data(app: &mut App) -> Result<(), AppError> {
    // Это также помещает хранилище в таблицу ресурсов приложения
    // Ваши следующие вызовы `store` как из Rust, так и от JS повторно используют то же хранилище
    let store = app.store("store.json")?;

    // Проверяем есть ли настройки в хранилище
    if store.is_empty() {
        create_new_store(app, store);
        Ok(())
    } else {
        read_store_data(app, store);
        Ok(())
    }
}
