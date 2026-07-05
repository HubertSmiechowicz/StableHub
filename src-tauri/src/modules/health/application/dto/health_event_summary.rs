use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HealthEventSummary {
    pub id: String,
    pub horse_id: String,
    pub horse_name: Option<String>,
    pub event_type: String,
    pub occurred_on: String,
    pub title: String,
    pub cost: Option<f64>,
    pub status: String,
}
