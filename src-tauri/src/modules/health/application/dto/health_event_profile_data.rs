use crate::modules::health::domain::HealthEvent;

#[derive(Debug, Clone)]
pub struct HealthEventProfileData {
    pub event: HealthEvent,
    pub occurred_time: Option<String>,
    pub notes: Option<String>,
    pub cost: Option<f64>,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
}
