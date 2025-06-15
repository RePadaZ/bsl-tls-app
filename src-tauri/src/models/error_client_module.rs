use anyhow::anyhow;
use tauri::ipc::InvokeError;
use tauri_plugin_store::Error as StoreError;
use thiserror::Error;

// Список ошибок приложений
#[derive(Debug, Error)]
pub enum ErrorClientModule {
    #[error("Ошибка чтения данных из настроек")]
    HotkeyNotConfigured,

    #[error("Ошибка хранилища настроек приложения: {0}")]
    Store(#[from] StoreError),

    #[error("Ошибка хранилища настроек приложения: {0}")]
    SaveStore(String),
}

impl From<ErrorClientModule> for InvokeError {
    fn from(error: ErrorClientModule) -> Self {
        InvokeError::from_anyhow(anyhow!(error.to_string()))
    }
}
