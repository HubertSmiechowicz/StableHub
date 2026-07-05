use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct InventoryItemSummary {
    pub id: String,
    pub name: String,
    pub unit: String,
    pub quantity: f64,
    pub minimum_quantity: Option<f64>,
    pub daily_usage: Option<f64>,
    pub days_remaining: Option<f64>,
    pub status: String,
}
