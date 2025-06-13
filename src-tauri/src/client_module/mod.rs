use crate::models::error_client_module::ErrorClientModule;
use crate::models::standard_setting::DEFAULT_SETTINGS;
use crate::utils::{default_setting_map, set_standard_settings};
use std::collections::HashMap;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

/// Получает настройки приложения для клиента и проверяет что все настройки совпадают.
/// Если какие-то настройки повреждены, тогда получает стандартные настройки для корректной работы.
pub fn get_data_settings(app: AppHandle) -> Result<HashMap<String, String>, ErrorClientModule> {
    // Подключаемся к хранилищу
    let store = if let Some(path_str) = DEFAULT_SETTINGS.get("settings.json") {
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

    // Создаем map в который будем помещать значения
    let mut map = HashMap::new();

    // Далее получаем значения по одному и помещаем их в map
    // Если какая-то настройка отсутствует или повреждена то тогда вставляем стандартные настройки
    for (key, default_value) in DEFAULT_SETTINGS.entries() {
        // Получаем сам объект
        let value = store.get(*key).unwrap_or_default();

        // Далее получаем из объекта значение и помещаем в map
        if let Some(value_str) = value.get("value").and_then(|v| v.as_str()) {
            if value_str.is_empty() {
                map.insert((*key).to_string(), (*default_value).to_string());
            } else {
                map.insert((*key).to_string(), (*value_str).to_string());
            }
        }
    }

    Ok(map)
}
