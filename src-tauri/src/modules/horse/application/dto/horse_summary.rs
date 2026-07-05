use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct HorseSummary {
    pub id: String,
    pub name: String,
    pub sex: Option<String>,
    pub breed: Option<String>,
    pub status: String,
}
