use crate::models::error::AppError;
use serde_json::json;
use std::str::FromStr;
use std::sync::Arc;
use tauri::{App, Wry};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};
use tauri_plugin_store::Store;

/// Создает новые записи в хранилище.
/// Обратите внимание, что значения должны быть serde_json::Value instances,
/// в противном случае они не будут совместимы с привязками JavaScript.
pub fn create_new_store(app: &mut App, store: Arc<Store<Wry>>) -> Result<(), AppError> {
    // Установка стандартных настроек для приложения
    store.set("hotkey", json!({ "value": "Ctrl+N" }));
    store.set("theme", json!({ "value": "light" }));
    store.set("language", json!({ "value":"ru"}));
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
    let shortcut_my = Shortcut::from_str(&config)?;

    app.handle()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |_app, shortcut, event| {
                    if shortcut == &shortcut_my {
                        match event.state() {
                            ShortcutState::Pressed => {
                                shortcut_state_pressed();
                            }
                            _ => {}
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

//TODO
fn shortcut_state_pressed() {
    println!("handler_global_shortcut_event");
}
