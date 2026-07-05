use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HealthEventDetails {
    pub id: String,
    pub horse_id: String,
    pub horse_name: Option<String>,
    pub event_type: String,
    pub occurred_on: String,
    pub title: String,
    pub notes: Option<String>,
    pub cost: Option<f64>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
}
