export type Settings = {
    hotkey: string;
    theme: "light" | "dark";
    language: "ru" | "en";
};

export type HotkeyInputProps = {
    value: string;
    onChange: (val: string) => void;
};