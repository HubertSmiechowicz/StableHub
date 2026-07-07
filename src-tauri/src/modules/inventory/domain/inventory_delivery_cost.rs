#[derive(Debug, Clone, Copy)]
pub struct InventoryDeliveryCost {
    value: f64,
}

impl InventoryDeliveryCost {
    pub fn new(value: f64) -> Result<Self, String> {
        if value < 0.0 {
            return Err("Koszt dostawy nie może być ujemny".to_string());
        }

        Ok(Self { value })
    }

    pub fn value(self) -> f64 {
        self.value
    }
}
