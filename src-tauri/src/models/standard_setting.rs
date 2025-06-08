use phf::phf_map;

pub static DEFAULT_SETTINGS: phf::Map<&'static str, &'static str> = phf_map! {
    "hotkey" => "Ctrl+N",
    "theme" => "light",
    "language" => "ru",
    "pathSetting" => "settings.json",
};
