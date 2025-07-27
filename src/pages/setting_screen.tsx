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
        <div className="min-h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900">
            <div className="flex items-center justify-center min-h-screen px-4">
                <div className="max-w-2xl w-full">
                    <div className="bg-white/5 backdrop-blur-xl rounded-3xl p-8 border border-white/10 shadow-2xl">
                        <div className="text-center mb-8">
                            <div
                                className="inline-flex items-center justify-center w-16 h-16 bg-gradient-to-r from-indigo-500 to-purple-600 rounded-2xl mb-6">
                                <svg className="w-8 h-8 text-white" fill="none" stroke="currentColor"
                                     viewBox="0 0 24 24">
                                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2}
                                          d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
                                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2}
                                          d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                                </svg>
                            </div>
                            <h1 className="text-4xl font-bold bg-gradient-to-r from-white to-gray-300 bg-clip-text text-transparent mb-4">
                                Настройки
                            </h1>
                            <p className="text-gray-300 text-lg">Настройте приложение под себя</p>
                        </div>

                        <div className="space-y-6">
                            <div className="bg-white/5 rounded-2xl p-6 border border-white/10">
                                <label className="flex items-center gap-2 text-white font-medium mb-4">
                                    Горячая клавиша
                                </label>
                                <HotkeyInput
                                    value={setting.hotkey}
                                    onChange={(val) => updateSetting("hotkey", val)}
                                />
                            </div>

                            <div className="bg-white/5 rounded-2xl p-6 border border-white/10">
                                <label className="flex items-center gap-2 text-white font-medium mb-4">
                                    <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2}
                                              d="M3 5h12M9 3v2m1.048 9.5A18.022 18.022 0 016.412 9m6.088 9h7M11 21l5-10 5 10M12.751 5C11.783 10.77 8.07 15.61 3 18.129"/>
                                    </svg>
                                    Язык интерфейса
                                </label>
                                <div className="flex gap-3">
                                    {(["ru", "en"] as const).map((lang) => (
                                        <label key={lang}
                                               className="flex items-center gap-3 px-4 py-3 bg-white/5 rounded-xl border border-white/10 cursor-pointer hover:bg-white/10 transition-all duration-200 flex-1">
                                            <input
                                                type="radio"
                                                name="language"
                                                value={lang}
                                                checked={setting.language === lang}
                                                onChange={() => updateSetting("language", lang)}
                                                className="text-indigo-500 focus:ring-indigo-500"
                                            />
                                            <span className="text-white font-medium">
                                                {lang === "ru" ? "Русский" : "English"}
                                            </span>
                                        </label>
                                    ))}
                                </div>
                            </div>
                        </div>

                        <div className="flex flex-col sm:flex-row gap-4 mt-8">
                            <button
                                onClick={() => invoke("save_data_setting", {setting})}
                                className="flex-1 px-6 py-3 bg-gradient-to-r from-indigo-500 to-purple-600 hover:from-indigo-600 hover:to-purple-700 rounded-xl text-white font-medium transition-all duration-200 shadow-lg hover:shadow-xl flex items-center justify-center gap-2"
                            >
                                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2}
                                          d="M8 7H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3-3m0 0l-3 3m3-3v12"/>
                                </svg>
                                Сохранить
                            </button>
                            <a
                                href="/"
                                className="flex-1 px-6 py-3 bg-white/10 hover:bg-white/20 rounded-xl text-white font-medium transition-all duration-200 border border-white/20 flex items-center justify-center gap-2"
                            >
                                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2}
                                          d="M10 19l-7-7m0 0l7-7m-7 7h18"/>
                                </svg>
                                Назад
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}
