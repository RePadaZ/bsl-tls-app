import {Settings} from "../Type.ts";
import {useState} from "react";
import {HotkeyInput} from "../component/HotkeyInput.tsx";

export function SettingScreen() {
    const [settings, setSettings] = useState<Settings>({
        hotkey: "Ctrl+N",
        theme: "light",
        language: "ru",
    });

    return (
        <div className="max-w-md mx-auto p-6 bg-white dark:bg-gray-800 rounded shadow-md">
            <h1 className="text-2xl font-bold mb-6 text-gray-900 dark:text-gray-100">
                Настройки
            </h1>
            <label className="block mb-4">
                <span className="text-gray-700 dark:text-gray-300">Горячая клавиша</span>
                <HotkeyInput
                    value={settings.hotkey}
                    onChange={(val) => setSettings({...settings, hotkey: val})}
                />
            </label>

            <fieldset className="mb-4">
                <legend className="text-gray-700 dark:text-gray-300 mb-2">Тема</legend>
                <label className="mr-4">
                    <input
                        type="radio"
                        name="theme"
                        value="light"
                        checked={settings.theme === "light"}
                        onChange={() => setSettings({...settings, theme: "light"})}
                        className="mr-1"
                    />
                    Светлая
                </label>
                <label>
                    <input
                        type="radio"
                        name="theme"
                        value="dark"
                        checked={settings.theme === "dark"}
                        onChange={() => setSettings({...settings, theme: "dark"})}
                        className="mr-1"
                    />
                    Тёмная
                </label>
            </fieldset>

            <fieldset className="mb-6">
                <legend className="text-gray-700 dark:text-gray-300 mb-2">Язык</legend>
                <label className="mr-4">
                    <input
                        type="radio"
                        name="language"
                        value="ru"
                        checked={settings.language === "ru"}
                        onChange={() => setSettings({...settings, language: "ru"})}
                        className="mr-1"
                    />
                    Русский
                </label>
                <label>
                    <input
                        type="radio"
                        name="language"
                        value="en"
                        checked={settings.language === "en"}
                        onChange={() => setSettings({...settings, language: "en"})}
                        className="mr-1"
                    />
                    English
                </label>
            </fieldset>

            <button
                // onClick={saveSettings}
                className="w-full bg-indigo-600 hover:bg-indigo-700 text-white py-2 rounded focus:outline-none focus:ring-2 focus:ring-indigo-500"
            >
                Сохранить
            </button>
        </div>
    );
}
