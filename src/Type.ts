export type AppState = "loading" | "ready" | "error";

export type Props = {
    message: string;
    onRetry: () => void;
};

export type Settings = {
    hotkey: string;
    theme: "light" | "dark";
    language: "ru" | "en";
};

export type HotkeyInputProps = {
    value: string;
    onChange: (val: string) => void;
};