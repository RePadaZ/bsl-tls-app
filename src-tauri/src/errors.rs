use thiserror::Error;
use serde::Serialize;

#[derive(Debug, Error, Serialize)]
pub enum AppError {
    #[error("Не найден BSL Server. Переустановите приложение.")]
    InvalidBslPath,

    // #[error("Произошла неизвестная ошибка")]
    // Unknown,
}
