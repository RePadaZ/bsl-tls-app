use tauri::utils::assets::phf::phf_map;

pub static DEFAULT_SETTINGS: phf::Map<&'static str, &'static str> = phf_map! {
    "hotkey" => "Ctrl+N",
    "language" => "ru",
    "path_setting" => "settings.json",
};
