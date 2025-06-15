export default function Main_screen() {

    return (
        <div
            className="min-h-screen bg-gradient-to-br from-indigo-600 via-purple-600 to-pink-500 flex items-center justify-center px-4">
            <div className="bg-white/10 backdrop-blur-lg rounded-2xl shadow-2xl p-10 max-w-xl w-full text-center">
                <h1 className="text-4xl md:text-5xl font-bold text-white mb-6">
                    Добро пожаловать в <span className="text-yellow-300">BSL TLS App</span>
                </h1>

                <p className="text-lg md:text-xl text-white/80 mb-8">
                    Надёжный помощник для работы с BSL-сервером.
                </p>

                <a
                    href="/setting"
                    className="bg-yellow-300 hover:bg-yellow-400 text-black font-semibold py-3 px-6 rounded-xl transition duration-300 shadow-md inline-block"
                >
                    ⚙️ Настройки
                </a>
            </div>
        </div>
    );
}
