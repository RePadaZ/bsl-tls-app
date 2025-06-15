use serde::{Deserialize, Serialize};
use serde_json::to_value;

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    pub hotkey: String,
    pub language: String,
    pub path_setting: String,
}

impl Settings {
    pub fn get(&self, key: &str) -> Option<String> {
        let value = to_value(self).ok()?;
        value.get(key)?.as_str().map(|s| s.to_string())
    }
}
