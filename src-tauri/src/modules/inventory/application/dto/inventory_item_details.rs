use serde::Serialize;

use super::{InventoryDeliverySummary, InventoryStocktakeSummary};

#[derive(Debug, Clone, Serialize)]
pub struct InventoryItemDetails {
    pub id: String,
    pub name: String,
    pub unit: String,
    pub quantity: f64,
    pub minimum_quantity: Option<f64>,
    pub daily_usage: Option<f64>,
    pub days_remaining: Option<f64>,
    pub status: String,
    pub recent_deliveries: Vec<InventoryDeliverySummary>,
    pub recent_stocktakes: Vec<InventoryStocktakeSummary>,
    pub total_delivery_cost: f64,
    pub average_unit_cost: Option<f64>,
    pub last_usage_applied_at: Option<String>,
    pub pending_usage_days: i64,
    pub pending_usage_quantity: f64,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
}
