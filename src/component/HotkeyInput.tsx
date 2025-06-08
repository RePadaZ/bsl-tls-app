import React, { useState } from "react";
import {HotkeyInputProps} from "../Type.ts";

// Функция для формирования строки комбинации клавиш
function formatShortcut(e: React.KeyboardEvent<HTMLInputElement>): string {

    // Массив горячей клавиши
    const keys: string[] = [];

    // Проверяем и добавляем нажатую клавишу в массив клавиш
    if (e.ctrlKey) keys.push("Ctrl");
    if (e.altKey) keys.push("Alt");
    if (e.shiftKey) keys.push("Shift");
    if (e.metaKey) keys.push("Meta");

    const key = e.key.toUpperCase();

    // Исключаем отдельные модификаторы из вывода
    if (!["CONTROL", "SHIFT", "ALT", "META"].includes(key)) {
        keys.push(key);
    }

    return keys.join("+");
}

export function HotkeyInput({ value, onChange }: HotkeyInputProps) {

    const [displayValue, setDisplayValue] = useState(value);

    const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
        e.preventDefault(); // Запретить ввод обычного текста

        const combo = formatShortcut(e);
        setDisplayValue(combo);
        onChange(combo);
    };

    return (
        <input
            type="text"
            value={displayValue}
            onKeyDown={handleKeyDown}
            onChange={() => {}} // Отключаем обычный ввод
            placeholder="Нажмите сочетание клавиш"
            className="mt-1 block w-full rounded border border-gray-300 bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 px-3 py-2 focus:outline-none focus:ring-2 focus:ring-indigo-500"
        />
    );
}
