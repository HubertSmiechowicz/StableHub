use crate::modules::inventory::application::{
    dto::InventoryItemDetails, ports::InventoryRepository,
};

pub struct GetInventoryItemDetailsQuery {
    pub id: String,
}

pub struct GetInventoryItemDetailsHandler<'a, R>
where
    R: InventoryRepository,
{
    repository: &'a R,
}

impl<'a, R> GetInventoryItemDetailsHandler<'a, R>
where
    R: InventoryRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        query: GetInventoryItemDetailsQuery,
    ) -> Result<InventoryItemDetails, String> {
        self.repository
            .find_details_by_id(&query.id)
            .await?
            .ok_or_else(|| "Nie znaleziono pozycji magazynowej".to_string())
    }
}
