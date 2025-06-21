use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, VIRTUAL_KEY, VK_A,
    VK_C, VK_CONTROL,
};

/// Функция для создания горячих клавиш.
/// Принимает в себя виртуальную клавишу и соединяет её с клавишей Ctrl
fn create_hotkey_windows(vk: VIRTUAL_KEY) -> [INPUT; 4] {
    let hotkey = [
        // Нажатие Ctrl
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_CONTROL,
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
                    wVk: vk,
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
                    wVk: vk,
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
                    wVk: VK_CONTROL,
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

pub unsafe fn shortcut_state_pressed() {
    let hotkey = create_hotkey_windows(VK_A);
    // Отправка Ctrl+A
    SendInput(&hotkey, std::mem::size_of::<INPUT>() as i32);

    // Небольшая задержка между нажатиями
    std::thread::sleep(std::time::Duration::from_millis(100));

    let hotkey = create_hotkey_windows(VK_C);
    // Отправка Ctrl+C
    SendInput(&hotkey, std::mem::size_of::<INPUT>() as i32);
}

//TODO
fn send_bufer_for_bsl() {}
