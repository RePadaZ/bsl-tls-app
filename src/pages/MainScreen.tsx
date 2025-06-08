export default function MainScreen() {

    return (
        <div className="bg-gradient-to-br from-indigo-600 via-purple-600 to-pink-500
            min-h-screen flex items-center justify-center text-white font-sans">
            <div className="bg-white/10 p-10 rounded-2xl shadow-2xl backdrop-blur-md
                max-w-md w-full text-center animate-fade-in">
                <h1 className="text-4xl font-bold mb-4">
                    Добро пожаловать в{" "}
                    <span className="text-yellow-300">BSL TLS App</span>
                </h1>
                <p className="text-lg mb-6 text-white/80">
                    Надёжный помощник для работы с BSL-сервером.
                </p>
                <div className={"justify-center"}>
                    <a className="cursor-pointer bg-yellow-300 hover:bg-yellow-400 text-black
                    font-semibold py-2 px-6 rounded-xl transition-all duration-300 mr-4">
                        Запустить
                    </a>
                    <a href={'/setting'} className="cursor-pointer mask-clip-content bg-yellow-300 hover:bg-yellow-400 text-black
                    font-semibold py-2 px-6 rounded-xl transition-all duration-300">
                        Настройки
                    </a>
                </div>
            </div>
        </div>
    );
}
