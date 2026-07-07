#[derive(Debug, Clone, Copy)]
pub struct InventoryDeliveryQuantity {
    value: f64,
}

impl InventoryDeliveryQuantity {
    pub fn new(value: f64) -> Result<Self, String> {
        if value <= 0.0 {
            return Err("Ilość dostawy musi być większa od zera".to_string());
        }

        Ok(Self { value })
    }

    pub fn value(self) -> f64 {
        self.value
    }
}
