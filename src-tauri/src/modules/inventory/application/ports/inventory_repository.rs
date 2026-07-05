use async_trait::async_trait;

use crate::modules::inventory::application::dto::{
    InventoryItemDetails, InventoryItemProfileData, InventoryItemSummary,
};

#[async_trait]
pub trait InventoryRepository {
    async fn save_item(&self, profile: &InventoryItemProfileData) -> Result<(), String>;
    async fn active_item_exists(&self, name: &str, unit: &str) -> Result<bool, String>;
    async fn list_active_summaries(&self) -> Result<Vec<InventoryItemSummary>, String>;
    async fn find_details_by_id(&self, id: &str) -> Result<Option<InventoryItemDetails>, String>;
}
