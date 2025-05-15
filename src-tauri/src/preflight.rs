use std::env;
use std::path::PathBuf;
use crate::errors::AppError;

pub fn check_bsl_server_path() -> Result<(), AppError> {
    // Получаем директорию проекта и добавляем путь до bsl server
    let path: PathBuf = env::current_dir().unwrap()
        .join("bsl-server").join("bsl-language-server.exe");

    // Проверяем есть ли данный файл
    if !path.exists() {
        drop(path);
        return Err(AppError::InvalidBslPath)
    }

    Ok(())
}
