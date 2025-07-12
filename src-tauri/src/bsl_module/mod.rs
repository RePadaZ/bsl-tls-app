use crate::models::error::AppError;
use crate::models::parse_json::Root;
use clipboard_win::get_clipboard_string;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};
use std::process::Command;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, VIRTUAL_KEY, VK_A,
    VK_C, VK_CONTROL,
};

/// Функция вызывается при нажатии горчих клавиш и возвращает структуру
pub unsafe fn shortcut_state_pressed() -> Result<Root, AppError> {
    let hotkey = create_hotkey_windows(VK_A, VK_CONTROL);
    // Отправка Ctrl+A
    SendInput(&hotkey, size_of::<INPUT>() as i32);

    // Небольшая задержка между нажатиями
    std::thread::sleep(std::time::Duration::from_millis(100));

    let hotkey = create_hotkey_windows(VK_C, VK_CONTROL);
    // Отправка Ctrl+C
    SendInput(&hotkey, size_of::<INPUT>() as i32);

    send_buffer_for_bsl()?;

    let data = read_generic_json()?;

    Ok(data)
}

fn send_buffer_for_bsl() -> Result<(), AppError> {
    let mut path = env::current_dir()?; // получаем текущую директорию
    path.push("bsl-language-server");
    path.push("file.os");

    if !path.exists() {
        File::create(&path)?;
    }

    let mut file = OpenOptions::new().write(true).create(true).open(&path)?;
    file.write_all(b"")?;

    match get_clipboard_string() {
        Ok(text) => {
            file.write_all(text.as_bytes())?;
        }
        Err(e) => {
            eprintln!("Ошибка при получении текста из буфера: {}", e);
            return Err(AppError::ErrorClipboard(e));
        }
    };

    path.pop();

    Command::new("powershell")
        .args([
            "-Command",
            "Start-Process",
            "bsl-language-server.exe",
            "-ArgumentList '-a -s . -r generic'",
            "-WorkingDirectory",
            path.to_str().unwrap(),
            "-Verb",
            "RunAs",
        ])
        .spawn()
        .expect("Не удалось запустить с правами администратора");

    Ok(())
}

fn read_generic_json() -> Result<Root, AppError> {
    let mut path = env::current_dir()?;
    path.push("bsl-language-server");
    path.push("bsl-generic-json.json");

    let file = OpenOptions::new().read(true).open(&path)?;

    let reader = BufReader::new(file);

    match serde_json::from_reader(reader) {
        Ok(data) => Ok(data),
        Err(e) => Err(AppError::ErrorReadJSON(e)),
    }
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
