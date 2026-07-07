use super::{InventoryItemId, InventoryItemName, InventoryItemStatus, InventoryUnit, StockLevel};

#[derive(Debug, Clone)]
pub struct InventoryItem {
    id: InventoryItemId,
    name: InventoryItemName,
    unit: InventoryUnit,
    stock_level: StockLevel,
    status: InventoryItemStatus,
}

impl InventoryItem {
    pub fn create(
        id: InventoryItemId,
        name: InventoryItemName,
        unit: InventoryUnit,
        stock_level: StockLevel,
    ) -> Self {
        Self {
            id,
            name,
            unit,
            stock_level,
            status: InventoryItemStatus::Active,
        }
    }

    pub fn from_existing(
        id: InventoryItemId,
        name: InventoryItemName,
        unit: InventoryUnit,
        stock_level: StockLevel,
        status: InventoryItemStatus,
    ) -> Self {
        Self {
            id,
            name,
            unit,
            stock_level,
            status,
        }
    }

    pub fn update_profile(&mut self, name: InventoryItemName, unit: InventoryUnit) {
        self.name = name;
        self.unit = unit;
    }

    pub fn archive(&mut self) {
        self.status = InventoryItemStatus::Archived;
    }

    pub fn id(&self) -> &InventoryItemId {
        &self.id
    }

    pub fn name(&self) -> &InventoryItemName {
        &self.name
    }

    pub fn unit(&self) -> InventoryUnit {
        self.unit
    }

    pub fn stock_level(&self) -> StockLevel {
        self.stock_level
    }

    pub fn status(&self) -> InventoryItemStatus {
        self.status
    }
}
