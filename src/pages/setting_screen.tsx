import {useEffect, useState} from "react";
import {HotkeyInput} from "../component/hotkey_input.tsx";
import {invoke} from "@tauri-apps/api/core";
import {Settings} from "../Type.ts";
import {labelClasses, optionClasses} from "../util/css.ts";

export function Setting_screen() {
    const [setting, setSettings] = useState<Settings>({
        hotkey: "Ctrl+N",
        language: "ru",
        path_setting: "settings.json"
    });
    useEffect(() => {
        async function loadSetting() {
            try {
                const result = await invoke<Partial<Settings>>("get_data_setting");
                if (result) {
                    setSettings(prev => ({
                        ...prev,
                        ...result
                    }));
                }
            } catch (error) {
                console.error("Ошибка при загрузке настроек:", error);
            }
        }

        loadSetting();
    }, []);

    const updateSetting = (key: keyof Settings, value: string) => {
        setSettings((prev) => ({...prev, [key]: value}));
    };

    return (
        <div className="min-h-screen bg-gray-100 dark:bg-gray-900 py-10 px-4 flex justify-center">
            <div className="w-full max-w-2xl bg-white dark:bg-gray-800 shadow-xl rounded-2xl p-8">
                <h1 className="text-3xl font-semibold text-center text-gray-900 dark:text-white mb-8">
                    ⚙️ Настройки
                </h1>

                {/* Горячая клавиша */}
                <div className="mb-6">
                    <label className={labelClasses}>Горячая клавиша</label>
                    <div
                        className="border rounded-lg p-3 bg-gray-50 dark:bg-gray-700 focus-within:ring-2 focus-within:ring-indigo-500">
                        <HotkeyInput
                            value={setting.hotkey}
                            onChange={(val) => updateSetting("hotkey", val)}
                        />
                    </div>
                </div>

                {/* Язык */}
                <div className="mb-8">
                    <label className={labelClasses}>Язык интерфейса</label>
                    <div className="flex gap-4">
                        {(["ru", "en"] as const).map((lang) => (
                            <label key={lang} className={optionClasses}>
                                <input
                                    type="radio"
                                    name="language"
                                    value={lang}
                                    checked={setting.language === lang}
                                    onChange={() => updateSetting("language", lang)}
                                />
                                <span className="text-gray-800 dark:text-gray-200">
                                    {lang === "ru" ? "Русский" : "English"}
                                </span>
                            </label>
                        ))}
                    </div>
                </div>

                {/* Кнопки */}
                <div className="flex flex-col sm:flex-row justify-between items-center gap-4">
                    <button
                        onClick={() => invoke("save_data_setting", {setting})}
                        className="w-full sm:w-auto px-6 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg
                        transition shadow focus:outline-none focus:ring-2 focus:ring-indigo-500 cursor-pointer"
                    >
                        💾 Сохранить
                    </button>
                    <a
                        href="/"
                        className="w-full sm:w-auto px-6 py-2 bg-gray-300 hover:bg-gray-400 text-gray-800 dark:bg-gray-700
                        dark:hover:bg-gray-600 dark:text-white rounded-lg transition shadow focus:outline-none focus:ring-2 focus:ring-gray-400"
                    >
                        ← Назад
                    </a>
                </div>
            </div>
        </div>
    );
}
