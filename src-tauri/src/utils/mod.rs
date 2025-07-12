pub(crate) use crate::bsl_module::shortcut_state_pressed;
use crate::models::error::AppError;
use crate::models::standard_setting::DEFAULT_SETTINGS;
use serde_json::json;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use tauri::{App, Emitter, Wry};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};
use tauri_plugin_store::Store;

/// Создает новые записи в хранилище.
/// Обратите внимание, что значения должны быть serde_json::Value instances,
/// в противном случае они не будут совместимы с привязками JavaScript.
pub fn create_new_store(app: &mut App, store: Arc<Store<Wry>>) -> Result<(), AppError> {
    // Установка стандартных настроек для приложения
    set_standard_settings(store);
    parse_and_register_shortcut(app, "Ctrl+N")?;
    Ok(())
}

/// Читает данные из хранилища и получает настройки приложения.
/// Для настроек приложения которые требуются в самом начале.
pub fn read_store_data(app: &mut App, store: Arc<Store<Wry>>) -> Result<(), AppError> {
    let hokey = store.get("hotkey").unwrap_or_default();
    if let Some(hotkey_str) = hokey.get("value").and_then(|v| v.as_str()) {
        parse_and_register_shortcut(app, &hotkey_str)?;
    } else {
        return Err(AppError::HotkeyNotConfigured);
    };
    Ok(())
}

/// Устанавливает глобальную клавишу из хранилища настроек приложения.
/// В самом начале запуска приложения.
fn parse_and_register_shortcut(app: &mut App, config: &str) -> Result<(), AppError> {
    // Парсим строку и делаем её тип Shortcut
    let shortcut_my = get_current_shortcut(&config)?;

    app.handle()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, shortcut, event| {
                    if shortcut == &shortcut_my {
                        if let ShortcutState::Pressed = event.state() {
                            unsafe {
                                match shortcut_state_pressed() {
                                    Ok(root) => {
                                        // Отправляем root на клиент
                                        app.emit("issues-update", root)
                                            .expect("TODO: panic message");
                                        // open_result_window(app, root);
                                    }
                                    Err(e) => {
                                        eprintln!("Ошибка при обработке: {:?}", e);
                                    }
                                }
                            }
                        }
                    }
                })
                .build(),
        )
        .expect("Handle global key error");

    // Регистрируем клавишу для работы
    app.global_shortcut()
        .register(shortcut_my)
        .expect("Failed to set the global key");
    Ok(())
}

/// Устанавливает стандартные настройки для приложения
pub fn set_standard_settings(store: Arc<Store<Wry>>) {
    for (key, value) in DEFAULT_SETTINGS.entries() {
        store.set(*key, json!({ "value": *value }));
    }
}

pub fn default_setting_map() -> Result<HashMap<String, String>, AppError> {
    let mut map = HashMap::new();
    for (key, value) in DEFAULT_SETTINGS.entries() {
        map.insert((*key).to_string(), (*value).to_string());
    }
    Ok(map)
}

/// Функция делает из строки горячую клавишу и возвращает результат
pub fn get_current_shortcut(hot_key: &str) -> Result<Shortcut, AppError> {
    let shortcut_my = Shortcut::from_str(&hot_key)?;
    Ok(shortcut_my)
}
