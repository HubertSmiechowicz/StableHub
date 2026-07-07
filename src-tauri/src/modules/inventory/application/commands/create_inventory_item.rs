use crate::modules::inventory::{
    application::{
        dto::{InventoryItemDetails, InventoryItemProfileData},
        ports::InventoryRepository,
    },
    domain::{InventoryItem, InventoryItemId, InventoryItemName, InventoryUnit, StockLevel},
};

pub struct CreateInventoryItemCommand {
    pub name: String,
    pub unit: String,
    pub minimum_quantity: Option<f64>,
    pub daily_usage: Option<f64>,
}

pub struct CreateInventoryItemHandler<'a, R>
where
    R: InventoryRepository,
{
    repository: &'a R,
}

impl<'a, R> CreateInventoryItemHandler<'a, R>
where
    R: InventoryRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: CreateInventoryItemCommand,
    ) -> Result<InventoryItemDetails, String> {
        let name = InventoryItemName::new(command.name).map_err(|error| error.to_string())?;
        let unit =
            InventoryUnit::try_from(command.unit.as_str()).map_err(|error| error.to_string())?;
        let stock_level = StockLevel::new(0.0).map_err(|error| error.to_string())?;
        let minimum_quantity = normalize_non_negative(command.minimum_quantity)?;
        let daily_usage = normalize_non_negative(command.daily_usage)?;

        let duplicate_exists = self
            .repository
            .active_item_exists(name.as_str(), unit.as_str(), None)
            .await?;

        if duplicate_exists {
            return Err(
                "Aktywna pozycja magazynowa o tej nazwie i jednostce już istnieje".to_string(),
            );
        }

        let item = InventoryItem::create(InventoryItemId::generate(), name, unit, stock_level);
        let now = current_timestamp();
        let profile = InventoryItemProfileData {
            item,
            minimum_quantity,
            daily_usage,
            created_at: now.clone(),
            updated_at: now.clone(),
            archived_at: None,
            last_usage_applied_at: Some(now),
        };

        self.repository.save_item(&profile).await?;

        Ok(InventoryItemDetails {
            id: profile.item.id().as_str().to_string(),
            name: profile.item.name().as_str().to_string(),
            unit: profile.item.unit().as_str().to_string(),
            quantity: profile.item.stock_level().value(),
            minimum_quantity: profile.minimum_quantity,
            daily_usage: profile.daily_usage,
            days_remaining: days_remaining(profile.item.stock_level().value(), profile.daily_usage),
            status: profile.item.status().as_str().to_string(),
            recent_deliveries: Vec::new(),
            recent_stocktakes: Vec::new(),
            total_delivery_cost: 0.0,
            average_unit_cost: None,
            last_usage_applied_at: profile.last_usage_applied_at,
            pending_usage_days: 0,
            pending_usage_quantity: 0.0,
            created_at: profile.created_at,
            updated_at: profile.updated_at,
            archived_at: profile.archived_at,
        })
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

fn days_remaining(quantity: f64, daily_usage: Option<f64>) -> Option<f64> {
    daily_usage
        .filter(|usage| *usage > 0.0)
        .map(|usage| quantity / usage)
}

fn current_timestamp() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}
