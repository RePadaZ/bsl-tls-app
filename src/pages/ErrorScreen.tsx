import {Props} from "../Type.ts";

export default function ErrorScreen({ message, onRetry }: Props) {
    return (
        <main className="container">
            <h1>❌ Ошибка</h1>
            <p>{message}</p>
            <button onClick={onRetry}>🔁 Повторить</button>
        </main>
    );
}
