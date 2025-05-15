type Props = {
    message: string;
    onRetry: () => void;
};

export default function ErrorScreen({ message, onRetry }: Props) {
    return (
        <main className="container">
            <h1>❌ Ошибка</h1>
            <p>{message}</p>
            <button onClick={onRetry}>🔁 Повторить</button>
        </main>
    );
}
