import {useEffect, useState} from "react";
import {HotkeyInput} from "../component/hotkey_input.tsx";
import {invoke} from "@tauri-apps/api/core";
import {Settings} from "../Type.ts";

export function Setting_screen() {

    const [setting, setSettings] = useState<Settings>({
        hotkey: "",
        language: "ru",
        path_setting: ""
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

        loadSetting().then(r => console.error(r));
    }, []);

    const updateSetting = (key: keyof Settings, value: string) => {
        setSettings((prev) => ({...prev, [key]: value}));
    };

    return (
        <div className="min-h-screen bg-gradient-to-br bg-gray-900 flex items-center justify-center px-4 ">
            <div className="w-full max-w-2xl bg-gray-800 shadow-xl rounded-2xl p-8 text-white">
                <h1 className="text-3xl font-semibold text-center mb-8">
                    ⚙️ Настройки
                </h1>

                {/* Горячая клавиша */}
                <div className="mb-6">
                    <label className="block text-sm font-medium mb-2">Горячая клавиша</label>
                    <HotkeyInput
                        value={setting.hotkey}
                        onChange={(val) => updateSetting("hotkey", val)}
                    />
                </div>

                {/* Язык */}
                <div className="mb-8">
                    <label className="block text-sm font-medium mb-2">Язык интерфейса</label>
                    <div className="flex gap-4">
                        {(["ru", "en"] as const).map((lang) => (
                            <label key={lang} className="flex items-center gap-2 px-4 py-2 bg-gray-700 rounded-lg
                                    cursor-pointer hover:bg-gray-600 transition">
                                <input
                                    type="radio"
                                    name="language"
                                    value={lang}
                                    checked={setting.language === lang}
                                    onChange={() => updateSetting("language", lang)}
                                />
                                <span>
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
                        className="w-full sm:w-auto px-6 py-2 bg-indigo-600 hover:bg-indigo-700 rounded-lg
                        transition shadow focus:outline-none focus:ring-2 focus:ring-indigo-500 cursor-pointer"
                    >
                        💾 Сохранить
                    </button>
                    <a
                        href="/"
                        className="w-full sm:w-auto px-6 py-2 bg-gray-700
                        hover:bg-gray-600 rounded-lg transition shadow focus:outline-none focus:ring-2 focus:ring-gray-400"
                    >
                        ← Назад
                    </a>
                </div>
            </div>
        </div>
    );
}
