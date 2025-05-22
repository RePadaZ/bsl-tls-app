use serde_json::json;
use std::sync::Arc;
use tauri::Wry;
use tauri_plugin_store::Store;

/// Создает новые записи в хранилище.
/// Обратите внимание, что значения должны быть serde_json::Value instances,
/// в противном случае они не будут совместимы с привязками JavaScript.
pub fn create_new_store(store: Arc<Store<Wry>>) {
    store.set("hotkey", json!({ "value": "Ctrl+N" }));
    store.set("theme", json!({ "value": "light" }));
    store.set("language", json!({ "value":"ru"}));
}
