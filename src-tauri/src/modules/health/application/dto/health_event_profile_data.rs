use crate::modules::health::domain::HealthEvent;

#[derive(Debug, Clone)]
pub struct HealthEventProfileData {
    pub event: HealthEvent,
    pub notes: Option<String>,
    pub cost: Option<f64>,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
}
