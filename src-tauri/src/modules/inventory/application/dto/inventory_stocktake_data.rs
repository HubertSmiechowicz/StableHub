#[derive(Debug, Clone)]
pub struct InventoryStocktakeData {
    pub id: String,
    pub inventory_item_id: String,
    pub counted_on: String,
    pub expected_quantity: f64,
    pub actual_quantity: f64,
    pub variance_quantity: f64,
    pub expected_usage: f64,
    pub notes: Option<String>,
    pub created_at: String,
}
