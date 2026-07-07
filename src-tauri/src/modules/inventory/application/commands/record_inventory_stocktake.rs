use crate::modules::inventory::{
    application::{
        dto::{InventoryItemDetails, InventoryStocktakeData},
        ports::InventoryRepository,
    },
    domain::{InventoryItemStatus, StockLevel},
};

pub struct RecordInventoryStocktakeCommand {
    pub inventory_item_id: String,
    pub counted_on: String,
    pub actual_quantity: f64,
    pub notes: Option<String>,
}

pub struct RecordInventoryStocktakeHandler<'a, R>
where
    R: InventoryRepository,
{
    repository: &'a R,
}

impl<'a, R> RecordInventoryStocktakeHandler<'a, R>
where
    R: InventoryRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: RecordInventoryStocktakeCommand,
    ) -> Result<InventoryItemDetails, String> {
        let profile = self
            .repository
            .find_profile_by_id(&command.inventory_item_id)
            .await?
            .ok_or_else(|| "Nie znaleziono pozycji magazynowej".to_string())?;

        if profile.item.status() == InventoryItemStatus::Archived {
            return Err(
                "Nie można wykonać inwentaryzacji usuniętej pozycji magazynowej".to_string(),
            );
        }

        let counted_on = command.counted_on.trim().to_string();
        if counted_on.is_empty() {
            return Err("Data inwentaryzacji jest wymagana".to_string());
        }

        let actual_quantity =
            StockLevel::new(command.actual_quantity).map_err(|error| error.to_string())?;

        let now = current_timestamp();
        let days = elapsed_days(profile.last_usage_applied_at.as_deref(), &now);
        let expected_usage = profile.daily_usage.unwrap_or(0.0).max(0.0) * days as f64;
        let current_quantity = profile.item.stock_level().value();
        let expected_quantity = (current_quantity - expected_usage).max(0.0);
        let variance_quantity = actual_quantity.value() - expected_quantity;

        let stocktake = InventoryStocktakeData {
            id: uuid::Uuid::new_v4().to_string(),
            inventory_item_id: profile.item.id().as_str().to_string(),
            counted_on,
            expected_quantity,
            actual_quantity: actual_quantity.value(),
            variance_quantity,
            expected_usage,
            notes: normalize_optional_text(command.notes),
            created_at: now,
        };

        self.repository.record_stocktake(&stocktake).await?;

        self.repository
            .find_details_by_id(profile.item.id().as_str())
            .await?
            .ok_or_else(|| "Nie znaleziono pozycji magazynowej po inwentaryzacji".to_string())
    }
}

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value
        .map(|text| text.trim().to_string())
        .filter(|text| !text.is_empty())
}

fn current_timestamp() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}

fn elapsed_days(last_applied_at: Option<&str>, now: &str) -> i64 {
    let Some(last_applied_at) = last_applied_at else {
        return 0;
    };

    let Ok(last) = last_applied_at.parse::<u64>() else {
        return 0;
    };
    let Ok(now) = now.parse::<u64>() else {
        return 0;
    };

    if now <= last {
        return 0;
    }

    ((now - last) / 86_400) as i64
}
