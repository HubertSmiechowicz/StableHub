use crate::modules::inventory::domain::InventoryItem;

#[derive(Debug, Clone)]
pub struct InventoryItemProfileData {
    pub item: InventoryItem,
    pub minimum_quantity: Option<f64>,
    pub daily_usage: Option<f64>,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
    pub last_usage_applied_at: Option<String>,
}
