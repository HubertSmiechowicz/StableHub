use crate::modules::inventory::{
    application::{
        dto::{InventoryDeliveryData, InventoryItemDetails},
        ports::InventoryRepository,
    },
    domain::{InventoryDeliveryCost, InventoryDeliveryQuantity, InventoryItemStatus},
};

pub struct RegisterInventoryDeliveryCommand {
    pub inventory_item_id: String,
    pub delivered_on: String,
    pub quantity: f64,
    pub total_cost: f64,
    pub supplier: Option<String>,
    pub notes: Option<String>,
}

pub struct RegisterInventoryDeliveryHandler<'a, R>
where
    R: InventoryRepository,
{
    repository: &'a R,
}

impl<'a, R> RegisterInventoryDeliveryHandler<'a, R>
where
    R: InventoryRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: RegisterInventoryDeliveryCommand,
    ) -> Result<InventoryItemDetails, String> {
        let profile = self
            .repository
            .find_profile_by_id(&command.inventory_item_id)
            .await?
            .ok_or_else(|| "Nie znaleziono pozycji magazynowej".to_string())?;

        if profile.item.status() == InventoryItemStatus::Archived {
            return Err(
                "Nie można rejestrować dostaw dla usuniętej pozycji magazynowej".to_string(),
            );
        }

        let delivered_on = command.delivered_on.trim().to_string();
        if delivered_on.is_empty() {
            return Err("Data dostawy jest wymagana".to_string());
        }

        let quantity = InventoryDeliveryQuantity::new(command.quantity)?;
        let total_cost = InventoryDeliveryCost::new(command.total_cost)?;

        let delivery = InventoryDeliveryData {
            id: uuid::Uuid::new_v4().to_string(),
            inventory_item_id: profile.item.id().as_str().to_string(),
            delivered_on,
            quantity: quantity.value(),
            total_cost: total_cost.value(),
            supplier: normalize_optional_text(command.supplier),
            notes: normalize_optional_text(command.notes),
            created_at: current_timestamp(),
        };

        self.repository.register_delivery(&delivery).await?;

        self.repository
            .find_details_by_id(&delivery.inventory_item_id)
            .await?
            .ok_or_else(|| "Nie znaleziono pozycji magazynowej po zapisaniu dostawy".to_string())
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
