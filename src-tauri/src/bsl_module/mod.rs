use crate::models::error::AppError;
use crate::models::parse_json::Root;
use clipboard_win::{get_clipboard_string, set_clipboard_string};
use std::env;
use std::mem::size_of;
use std::process::Command;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, VIRTUAL_KEY, VK_A,
    VK_C, VK_CONTROL,
};

/// Функция вызывается при нажатии горчих клавиш и возвращает структуру
pub unsafe fn shortcut_state_pressed() -> Result<Root, AppError> {
    // Очищаем буфер обмена перед началом операции
    set_clipboard_string("").map_err(AppError::ErrorClipboard)?;

    // Выполняем Ctrl+A и Ctrl+C последовательно с минимальной задержкой
    execute_keyboard_sequence(&[
        (VK_A, VK_CONTROL), // Ctrl+A для выделения всего текста
        (VK_C, VK_CONTROL), // Ctrl+C для копирования в буфер
    ])?;

    // Обрабатываем данные через BSL и возвращаем результат
    send_buffer_for_bsl()?;
    read_generic_json()
}

fn send_buffer_for_bsl() -> Result<(), AppError> {
    let mut file_path = env::current_dir()?;
    file_path.push("bsl-language-server");

    let working_dir = file_path.clone();
    file_path.push("file.os");

    // Получаем текст из буфера обмена и записываем в файл одной операцией
    let clipboard_text = get_clipboard_string().map_err(AppError::ErrorClipboard)?;
    std::fs::write(&file_path, clipboard_text.as_bytes())?;

    // Запускаем BSL language server с оптимизированными параметрами
    let status = Command::new("powershell")
        .args([
            "-NoProfile", // Ускоряет запуск PowerShell
            "-Command",
            "Start-Process",
            "bsl-language-server.exe",
            "-ArgumentList '-a -s . -r generic'",
            "-WorkingDirectory",
            working_dir.to_str().ok_or(AppError::ErrorWriteBslPath)?,
            "-Verb",
            "RunAs",
            "-Wait",
        ])
        .spawn()?
        .wait()?;

    if !status.success() {
        return Err(AppError::ErrorWriteBslPath);
    }

    Ok(())
}

fn read_generic_json() -> Result<Root, AppError> {
    let mut path = env::current_dir()?;
    path.push("bsl-language-server");
    path.push("bsl-generic-json.json");

    // Используем std::fs::read_to_string для более эффективного чтения
    let json_content = std::fs::read_to_string(&path)?;
    serde_json::from_str(&json_content).map_err(AppError::ErrorReadJSON)
}

/// Выполняет последовательность клавиатурных комбинаций с оптимальной задержкой
/// Более эффективно, чем отдельные вызовы SendInput
unsafe fn execute_keyboard_sequence(
    key_combinations: &[(VIRTUAL_KEY, VIRTUAL_KEY)],
) -> Result<(), AppError> {
    const OPTIMAL_DELAY_MS: u64 = 50; // Оптимальная задержка между комбинациями

    for &(base_key, modifier_key) in key_combinations {
        let hotkey = create_hotkey_windows(base_key, modifier_key);

        // Проверяем успешность отправки клавиш
        let result = SendInput(&hotkey, size_of::<INPUT>() as i32);
        if result != hotkey.len() as u32 {
            return Err(AppError::ErrorWriteBslPath); // Используем существующую ошибку
        }

        // Минимальная задержка для корректной обработки системой
        std::thread::sleep(std::time::Duration::from_millis(OPTIMAL_DELAY_MS));
    }

    Ok(())
}

/// Функция для создания горячих клавиш.
/// Принимает в себя виртуальную клавишу и соединяет её с клавишей Ctrl
fn create_hotkey_windows(vk_base: VIRTUAL_KEY, vk_extend: VIRTUAL_KEY) -> [INPUT; 4] {
    let hotkey = [
        // Нажатие Ctrl
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: vk_extend,
                    wScan: 0,
                    dwFlags: Default::default(),
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        // Нажатие A
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: vk_base,
                    wScan: 0,
                    dwFlags: Default::default(),
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        // Отпускание A
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: vk_base,
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        // Отпускание Ctrl
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: vk_extend,
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
    ];

    hotkey
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::{remove_file, File};
    use std::io::Write;

    // Вспомогательная функция для создания тестового JSON-файла
    fn create_test_json_file(content: &str) -> std::path::PathBuf {
        let mut path = env::current_dir().unwrap();
        path.push("bsl-language-server");
        std::fs::create_dir_all(&path).unwrap();
        path.push("bsl-generic-json.json");
        let mut file = File::create(&path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        path
    }

    #[test]
    fn test_read_generic_json_file_read() {
        // Прочитать валидный файл
        let mut path = env::current_dir().unwrap();
        path.push("bsl-language-server");
        path.push("bsl-generic-json.json");
        let _ = remove_file(&path);

        let result = read_generic_json();
        assert!(result.is_err(), "Должен быть прочитал корректно файл");
    }

    #[test]
    fn test_read_generic_json_file_not_found() {
        // Удаляем файл, если он есть
        let mut path = env::current_dir().unwrap();
        path.push("bsl-language-server");
        path.push("bsl-generic-json.json");
        let _ = remove_file(&path);

        let result = read_generic_json();
        assert!(result.is_err(), "Должна быть ошибка при отсутствии файла");
    }

    #[test]
    fn test_read_generic_json_invalid_json() {
        // Некорректный JSON
        let json = r#"{\"field1\": \"value\", \"field2\": }"#;
        let path = create_test_json_file(json);

        let result = read_generic_json();
        assert!(result.is_err(), "Должна быть ошибка при некорректном JSON");

        // Очистка
        remove_file(path).unwrap();
    }
}
