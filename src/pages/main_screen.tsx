export default function Main_screen() {
    return (
        <div
            className="min-h-screen bg-gradient-to-br bg-gray-900 flex items-center justify-center px-4">
            <div className="bg-white/10 rounded-2xl p-10 max-w-xl w-full text-center text-white">
                <h1 className="text-4xl font-bold mb-6">
                    Добро пожаловать в BSL TLS App
                </h1>

                <p className="text-lg mb-8">
                    Надёжный помощник для работы с BSL-сервером.
                </p>

                <a
                    href="/setting"
                    className="w-full px-6 py-4 bg-indigo-600 hover:bg-indigo-700 rounded-lg
                        transition cursor-pointer"
                >
                    ⚙️ Настройки
                </a>
            </div>
        </div>
    );
}
