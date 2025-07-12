import {listen} from "@tauri-apps/api/event";
import {useEffect, useState} from "react";

interface Root {
    issues: {
        primaryLocation: {
            message: string;
            textRange: {
                startLine: number;
            };
        };
    }[];
}


export default function Main_screen() {
    const [issues, setIssues] = useState<Root | null>(null);


    useEffect(() => {
        // Подписываемся на событие
        let unlisten: () => void;

        listen<Root>("issues-update", (event) => {
            console.log("Получены issues:", event.payload);
            setIssues(event.payload); // сохраняем данные в состояние
        }).then((f) => {
            unlisten = f;
        });

        // Отписка при размонтировании
        return () => {
            if (unlisten) unlisten();
        };
    }, []);

    return (
        <div className="min-h-screen bg-gradient-to-br bg-gray-900 flex items-center justify-center px-4">
            <div className="bg-white/10 rounded-2xl p-10 max-w-xl w-full text-center text-white">
                <h1 className="text-4xl font-bold mb-6">Добро пожаловать в BSL TLS App</h1>
                <p className="text-lg mb-8">
                    Надёжный помощник для работы с BSL-сервером.
                </p>

                <a
                    href="/setting"
                    className="w-full px-6 py-4 bg-indigo-600 hover:bg-indigo-700 rounded-lg transition cursor-pointer"
                >
                    ⚙️ Настройки
                </a>

                {issues && (
                    <div className="mt-8 text-left max-h-64 overflow-auto bg-white/20 p-4 rounded-lg text-white">
                        <h2 className="text-xl mb-4">Issues:</h2>
                        <ul>
                            {issues.issues.map((issue, idx) => (
                                <li key={idx} className="mb-2">
                                    <b>Сообщение:</b> {issue.primaryLocation.message} <br/>
                                    <b>Строка:</b> {issue.primaryLocation.textRange.startLine}
                                </li>
                            ))}
                        </ul>
                    </div>
                )}
            </div>
        </div>
    );
}
