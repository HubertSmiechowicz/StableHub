use crate::modules::inventory::application::{
    dto::InventoryItemSummary, ports::InventoryRepository,
};

pub struct ListInventoryItemsQuery;

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
        _query: ListInventoryItemsQuery,
    ) -> Result<Vec<InventoryItemSummary>, String> {
        self.repository.list_active_summaries().await
    }
}
