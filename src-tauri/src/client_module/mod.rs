use crate::models::custom_type::Settings;
use crate::models::error::AppError;
use crate::models::standard_setting::DEFAULT_SETTINGS;
use crate::utils::{
    default_setting_map, get_current_shortcut, set_standard_settings, shortcut_state_pressed,
};
use serde_json::json;
use std::collections::HashMap;
use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};
use tauri_plugin_store::Store;
use tauri_plugin_store::StoreExt;

/// Получает настройки приложения для клиента и проверяет что все настройки совпадают.
/// Если какие-то настройки повреждены, тогда получает стандартные настройки для корректной работы.
pub fn get_data_setting(app: AppHandle) -> Result<HashMap<String, String>, AppError> {
    // Подключаемся к хранилищу
    let store = if let Some(path_str) = DEFAULT_SETTINGS.get("path_setting") {
        app.store(path_str)?
    } else {
        // Создаем новое хранилище по умолчанию
        app.store("settings.json")?
    };

    // Если хранилище пустое, то заполняем его и возвращаем готовый map
    if store.is_empty() {
        set_standard_settings(store);
        return default_setting_map();
    }

    let map = get_the_values_settings(&store)?;
    Ok(map)
}

fn get_the_values_settings(store: &Store<tauri::Wry>) -> Result<HashMap<String, String>, AppError> {
    let mut map = HashMap::new();

    for (key, default_value) in DEFAULT_SETTINGS.entries() {
        let value = store.get(*key).unwrap_or_default();

        let setting_value = value
            .get("value")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .unwrap_or(default_value);

        map.insert(key.to_string(), setting_value.to_string());
    }

    Ok(map)
}

/// Сохраняет настройки, установленные пользователем.
/// В случае ошибки записи будет возвращена ошибка на клиент.
pub fn save_data_setting(app: AppHandle, setting: Settings) -> Result<(), AppError> {
    // Подключаемся к хранилищу
    let store = if let Some(path_str) = DEFAULT_SETTINGS.get("path_setting") {
        app.store(path_str)?
    } else {
        // Ошибка о создании
        return Err(AppError::SaveStore(
            "Не найдены настройки или нет доступа".into(),
        ));
    };

    for (key, _) in DEFAULT_SETTINGS.entries() {
        if let Some(value) = setting.get(*key) {
            if *key == "hotkey" {
                println!("{}", value);
                update_shortcut(app.clone(), value.as_str())?;
            }
            println!("{}", value);
            store.set(*key, json!({ "value": value }));
        }
    }

    Ok(())
}

/// Функция для обновления горячих клавиш из клиентской части. Очищает данные о горячих клавишах
/// после чего устанавливает новое значение.
fn update_shortcut(app: AppHandle, config: &str) -> Result<(), AppError> {
    // Парсим строку и делаем её тип Shortcut
    let shortcut_my = get_current_shortcut(&config)?;

    // Удаляем все обработчики и добавляем новый
    app.global_shortcut()
        .unregister_all()
        .expect("TODO: panic message");

    app.global_shortcut()
        .on_shortcut(shortcut_my, move |_app, shortcut, event| {
            if shortcut == &shortcut_my {
                match event.state() {
                    ShortcutState::Pressed => unsafe {
                        let _ = shortcut_state_pressed();
                    },
                    _ => {}
                }
            }
        })
        .map_err(|e| AppError::SetNewHotKey(e))?;

    Ok(())
}
