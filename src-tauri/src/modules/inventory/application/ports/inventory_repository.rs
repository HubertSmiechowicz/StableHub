use async_trait::async_trait;

use crate::modules::inventory::application::dto::{
    InventoryDeliveryData, InventoryItemDetails, InventoryItemProfileData, InventoryItemSummary,
    InventoryStocktakeData,
};

#[derive(Debug, Clone)]
pub struct InventoryListOptions {
    pub search: Option<String>,
    pub sort_by: InventorySortBy,
    pub sort_direction: SortDirection,
}

#[derive(Debug, Clone, Copy)]
pub enum InventorySortBy {
    Name,
    Quantity,
    DaysRemaining,
    CreatedAt,
}

#[derive(Debug, Clone, Copy)]
pub enum SortDirection {
    Asc,
    Desc,
}

#[async_trait]
pub trait InventoryRepository {
    async fn save_item(&self, profile: &InventoryItemProfileData) -> Result<(), String>;
    async fn find_profile_by_id(
        &self,
        id: &str,
    ) -> Result<Option<InventoryItemProfileData>, String>;
    async fn active_item_exists(
        &self,
        name: &str,
        unit: &str,
        except_id: Option<&str>,
    ) -> Result<bool, String>;
    async fn list_active_summaries(
        &self,
        options: &InventoryListOptions,
    ) -> Result<Vec<InventoryItemSummary>, String>;
    async fn find_details_by_id(&self, id: &str) -> Result<Option<InventoryItemDetails>, String>;
    async fn register_delivery(&self, delivery: &InventoryDeliveryData) -> Result<(), String>;
    async fn apply_usage(
        &self,
        item_id: &str,
        new_quantity: f64,
        applied_at: &str,
    ) -> Result<(), String>;
    async fn record_stocktake(&self, stocktake: &InventoryStocktakeData) -> Result<(), String>;
}
