use crate::modules::inventory::{
    application::ports::InventoryRepository, domain::InventoryItemStatus,
};

pub struct ArchiveInventoryItemCommand {
    pub id: String,
}

pub struct ArchiveInventoryItemHandler<'a, R>
where
    R: InventoryRepository,
{
    repository: &'a R,
}

impl<'a, R> ArchiveInventoryItemHandler<'a, R>
where
    R: InventoryRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, command: ArchiveInventoryItemCommand) -> Result<(), String> {
        let mut profile = self
            .repository
            .find_profile_by_id(&command.id)
            .await?
            .ok_or_else(|| "Nie znaleziono pozycji magazynowej".to_string())?;

        if profile.item.status() == InventoryItemStatus::Archived {
            return Ok(());
        }

        let now = current_timestamp();
        profile.item.archive();
        profile.updated_at = now.clone();
        profile.archived_at = Some(now);

        self.repository.save_item(&profile).await
    }
}

fn current_timestamp() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}
