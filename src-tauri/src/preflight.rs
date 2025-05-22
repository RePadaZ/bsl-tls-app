use crate::models::error::{AppError, LocalStoreType};
use crate::utils::create_new_store;
use std::env;
use std::path::PathBuf;
use std::str::FromStr;
use tauri::App;
use tauri_plugin_global_shortcut::{Error, GlobalShortcutExt, Shortcut, ShortcutState};
use tauri_plugin_store::StoreExt;

// Функция для проверки установки BSL Server
pub fn check_bsl_server_path() -> Result<(), AppError> {
    // Получаем директорию проекта и добавляем путь до bsl server
    let path: PathBuf = env::current_dir()
        .unwrap()
        .join("bsl-language-server")
        .join("bsl-language-server.exe");

    // Проверяем есть ли данный файл
    if !path.exists() {
        drop(path);
        return Err(AppError::InvalidBslPath);
    }

    Ok(())
}

// Функция для хранения данных пользователя
pub fn check_local_store_data(app: &mut App) -> Result<LocalStoreType, Error> {
    // Создайте новое хранилище или загрузить существующие
    // Это также помещает хранилище в таблицу ресурсов приложения
    // Ваши следующие вызовы `store` как из Rust, так и от JS повторно используют то же хранилище
    let store = app.store("store.json").expect("create or load store error");

    // Проверяем есть ли настройки в хранилище
    if store.is_empty() {
        create_new_store(store);
        Ok(LocalStoreType::Ok)
    } else {
        let mut array_setting = [const { String::new() }; 3];

        if let Some(hotkey) = store.get("hotkey").unwrap().get("value") {
            array_setting[0] = hotkey.to_string();
            let str_string: &str = hotkey.as_str().unwrap();
            parse_and_register_shortcut(app, &str_string);
        };
        if let Some(theme) = store.get("theme").unwrap().get("value") {
            array_setting[1] = theme.to_string();
        };
        if let Some(language) = store.get("language").unwrap().get("value") {
            array_setting[2] = language.to_string();
        };
        Ok(LocalStoreType::Values(array_setting))
    }
}

fn parse_and_register_shortcut(app: &mut App, config: &str) {
    let shortcut_my: Shortcut = Shortcut::from_str(&config).unwrap();
    app.handle()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |_app, shortcut, event| {
                    if shortcut == &shortcut_my {
                        match event.state() {
                            ShortcutState::Pressed => {
                                println!("Ctrl-N Pressed!");
                            }
                            _ => {}
                        }
                    }
                })
                .build(),
        )
        .expect("TODO: panic message");

    app.global_shortcut()
        .register(shortcut_my)
        .expect("shortcut");
}

// // Функция для установки горячих клавиш
// pub fn global_short(app: &mut App) -> Result<(), Error> {
//     // Задаем значение
//     let ctrl_n_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyN);
//     app.handle().plugin(
//         tauri_plugin_global_shortcut::Builder::new()
//             .with_handler(move |_app, shortcut, event| {
//                 if shortcut == &ctrl_n_shortcut {
//                     match event.state() {
//                         ShortcutState::Pressed => {
//                             println!("Ctrl-N Pressed!");
//                         }
//                         _ => {}
//                     }
//                 }
//             })
//             .build(),
//     )?;
//     app.global_shortcut().register(ctrl_n_shortcut)?;
//
//     Ok(())
// }
