import {listen} from "@tauri-apps/api/event";
import {useEffect, useState} from "react";
import {Root} from "../models/mod.ts";
import {SettingsButton} from "../component/settings_button.tsx";

export default function Main_screen() {
    const [issues, setIssues] = useState<Root | null>(null);

    useEffect(() => {
        let cleanup: (() => void) | undefined;

        (async () => {
            try {
                cleanup = await listen<Root>("issues-update", (event) => {
                    setIssues(event.payload);
                });
            } catch (error) {
                console.error(error);
            }
        })();

        return () => cleanup?.();
    }, []);


    return (
        <div className="min-h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900">
            <div className="flex items-center justify-center min-h-screen px-4">
                <div className="max-w-4xl w-full relative">
                    <div className="bg-white/5 backdrop-blur-xl rounded-3xl p-8 border border-white/10 shadow-2xl">
                        <SettingsButton/>
                        <div className="text-center mb-8">
                            <div
                                className="inline-flex items-center justify-center w-16 h-16 bg-gradient-to-r from-blue-500 to-purple-600 rounded-2xl mb-6">
                                <svg className="w-8 h-8 text-white" fill="none" stroke="currentColor"
                                     viewBox="0 0 24 24">
                                    <circle cx="12" cy="12" r="10"/>
                                    <path strokeLinecap="round" strokeLinejoin="round" d="m9 9h.01"/>
                                    <path strokeLinecap="round" strokeLinejoin="round" d="m15 9h.01"/>
                                    <path strokeLinecap="round" strokeLinejoin="round" d="m8 13a4 4 0 0 0 8 0"/>
                                </svg>
                            </div>
                            <h1 className="text-5xl font-bold bg-gradient-to-r from-white to-gray-300 bg-clip-text text-transparent mb-4">
                                Анализатор кода 1С
                            </h1>
                            <p className="text-xl text-gray-300 max-w-2xl mx-auto leading-relaxed">
                                Современный инструмент для анализа и проверки 1С кода
                            </p>
                        </div>

                        {issues && (
                            <div className="bg-white/5 rounded-2xl p-6 border border-white/10">
                                <div className="flex items-center gap-3 mb-6">
                                    <div className="w-2 h-2 bg-red-500 rounded-full animate-pulse"></div>
                                    <h2 className="text-2xl font-semibold text-white">
                                        Найденные проблемы
                                    </h2>
                                    <span
                                        className="bg-red-500/20 text-red-300 px-3 py-1 rounded-full text-sm font-medium">
                                        {issues.issues.length}
                                    </span>
                                </div>
                                <div className="max-h-96 overflow-auto space-y-3">
                                    {issues.issues.map((issue, idx) => (
                                        <div key={idx}
                                             className="bg-white/5 rounded-xl p-4 border border-white/10 hover:bg-white/10 transition-all duration-200">
                                            <div className="flex items-start gap-3">
                                                <div
                                                    className="w-2 h-2 bg-yellow-400 rounded-full mt-2 flex-shrink-0"></div>
                                                <div className="flex-1">
                                                    <p className="text-white font-medium mb-2">
                                                        {issue.primaryLocation.message}
                                                    </p>
                                                    <div className="flex items-center gap-2 text-sm text-gray-400">
                                                        <svg className="w-4 h-4" fill="none" stroke="currentColor"
                                                             viewBox="0 0 24 24">
                                                            <path strokeLinecap="round" strokeLinejoin="round"
                                                                  strokeWidth={2}
                                                                  d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z"/>
                                                        </svg>
                                                        Строка {issue.primaryLocation.textRange.startLine}
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    ))}
                                </div>
                            </div>
                        )}

                        {!issues?.issues?.length && (
                            <div className="text-center py-12">
                                <p className="text-gray-400 text-lg">Используйте горячие клавиши для анализа кода</p>
                            </div>
                        )}
                    </div>
                </div>
            </div>
        </div>
    );
}
