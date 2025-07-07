use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Root {
    pub issues: Vec<Issue>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    engine_id: String,
    rule_id: String,
    severity: String,
    #[serde(rename = "type")]
    issue_type: String,
    pub primary_location: PrimaryLocation,
    effort_minutes: u32,
    secondary_locations: Vec<SecondaryLocation>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryLocation {
    pub message: String,
    pub file_path: String,
    pub text_range: TextRange,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextRange {
    pub start_line: u32,
    end_line: u32,
    start_column: u32,
    end_column: u32,
}

#[derive(Debug, Deserialize)]
struct SecondaryLocation {
    // Если есть поля — добавь сюда, пока оставим пустым
}
