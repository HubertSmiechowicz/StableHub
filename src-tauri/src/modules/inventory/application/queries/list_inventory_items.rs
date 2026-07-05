use crate::modules::inventory::application::{
    dto::InventoryItemSummary,
    ports::{InventoryListOptions, InventoryRepository, InventorySortBy, SortDirection},
};

pub struct ListInventoryItemsQuery {
    pub search: Option<String>,
    pub sort_by: String,
    pub sort_direction: String,
}

pub struct ListInventoryItemsHandler<'a, R>
where
    R: InventoryRepository,
{
    repository: &'a R,
}

impl<'a, R> ListInventoryItemsHandler<'a, R>
where
    R: InventoryRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        query: ListInventoryItemsQuery,
    ) -> Result<Vec<InventoryItemSummary>, String> {
        let options = InventoryListOptions {
            search: normalize_search(query.search),
            sort_by: match query.sort_by.as_str() {
                "quantity" => InventorySortBy::Quantity,
                "days_remaining" => InventorySortBy::DaysRemaining,
                "created_at" => InventorySortBy::CreatedAt,
                _ => InventorySortBy::Name,
            },
            sort_direction: match query.sort_direction.as_str() {
                "desc" => SortDirection::Desc,
                _ => SortDirection::Asc,
            },
        };

        self.repository.list_active_summaries(&options).await
    }
}

fn normalize_search(search: Option<String>) -> Option<String> {
    search
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}
