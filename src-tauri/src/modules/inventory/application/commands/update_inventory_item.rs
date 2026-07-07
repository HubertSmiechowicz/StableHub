use crate::modules::inventory::{
    application::{dto::InventoryItemDetails, ports::InventoryRepository},
    domain::{InventoryItemName, InventoryItemStatus, InventoryUnit},
};

pub struct UpdateInventoryItemCommand {
    pub id: String,
    pub name: String,
    pub unit: String,
    pub minimum_quantity: Option<f64>,
    pub daily_usage: Option<f64>,
}

pub struct UpdateInventoryItemHandler<'a, R>
where
    R: InventoryRepository,
{
    repository: &'a R,
}

impl<'a, R> UpdateInventoryItemHandler<'a, R>
where
    R: InventoryRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: UpdateInventoryItemCommand,
    ) -> Result<InventoryItemDetails, String> {
        let mut profile = self
            .repository
            .find_profile_by_id(&command.id)
            .await?
            .ok_or_else(|| "Nie znaleziono pozycji magazynowej".to_string())?;

        if profile.item.status() == InventoryItemStatus::Archived {
            return Err("Nie można edytować usuniętej pozycji magazynowej".to_string());
        }

        let name = InventoryItemName::new(command.name).map_err(|error| error.to_string())?;
        let unit =
            InventoryUnit::try_from(command.unit.as_str()).map_err(|error| error.to_string())?;
        let minimum_quantity = normalize_non_negative(command.minimum_quantity)?;
        let daily_usage = normalize_non_negative(command.daily_usage)?;

        let duplicate_exists = self
            .repository
            .active_item_exists(
                name.as_str(),
                unit.as_str(),
                Some(profile.item.id().as_str()),
            )
            .await?;

        if duplicate_exists {
            return Err(
                "Aktywna pozycja magazynowa o tej nazwie i jednostce już istnieje".to_string(),
            );
        }

        profile.item.update_profile(name, unit);
        profile.minimum_quantity = minimum_quantity;
        profile.daily_usage = daily_usage;
        profile.updated_at = current_timestamp();

        self.repository.save_item(&profile).await?;

        self.repository
            .find_details_by_id(profile.item.id().as_str())
            .await?
            .ok_or_else(|| "Nie znaleziono pozycji magazynowej po aktualizacji".to_string())
    }
}

fn normalize_non_negative(value: Option<f64>) -> Result<Option<f64>, String> {
    match value {
        Some(value) if value < 0.0 => {
            Err("Wartości ilościowe magazynu nie mogą być ujemne".to_string())
        }
        Some(value) if value == 0.0 => Ok(None),
        other => Ok(other),
    }
}

fn current_timestamp() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}
