export type Settings = {
    hotkey: string;
    language: "ru" | "en";
    path_setting: string;
};

export type HotkeyInputProps = {
    value: string;
    onChange: (val: string) => void;
};