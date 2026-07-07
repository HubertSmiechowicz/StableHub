#[derive(Debug, Clone)]
pub struct InventoryDeliveryData {
    pub id: String,
    pub inventory_item_id: String,
    pub delivered_on: String,
    pub quantity: f64,
    pub total_cost: f64,
    pub supplier: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
}
