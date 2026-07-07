use crate::modules::inventory::{
    application::{dto::InventoryItemDetails, ports::InventoryRepository},
    domain::InventoryItemStatus,
};

pub struct ApplyInventoryUsageCommand {
    pub inventory_item_id: String,
}

pub struct ApplyInventoryUsageHandler<'a, R>
where
    R: InventoryRepository,
{
    repository: &'a R,
}

impl<'a, R> ApplyInventoryUsageHandler<'a, R>
where
    R: InventoryRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: ApplyInventoryUsageCommand,
    ) -> Result<InventoryItemDetails, String> {
        let profile = self
            .repository
            .find_profile_by_id(&command.inventory_item_id)
            .await?
            .ok_or_else(|| "Nie znaleziono pozycji magazynowej".to_string())?;

        if profile.item.status() == InventoryItemStatus::Archived {
            return Err("Nie można rozliczyć zużycia usuniętej pozycji magazynowej".to_string());
        }

        let now = current_timestamp();
        let days = elapsed_days(profile.last_usage_applied_at.as_deref(), &now);
        let daily_usage = profile.daily_usage.unwrap_or(0.0);

        if daily_usage <= 0.0 {
            return Err("Najpierw ustaw dzienne zużycie pozycji magazynowej".to_string());
        }

        if days <= 0 {
            return Err("Brak pełnych dni do rozliczenia".to_string());
        }

        let usage = daily_usage * days as f64;
        let current_quantity = profile.item.stock_level().value();
        let new_quantity = (current_quantity - usage).max(0.0);

        self.repository
            .apply_usage(profile.item.id().as_str(), new_quantity, &now)
            .await?;

        self.repository
            .find_details_by_id(profile.item.id().as_str())
            .await?
            .ok_or_else(|| "Nie znaleziono pozycji magazynowej po rozliczeniu zużycia".to_string())
    }
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
