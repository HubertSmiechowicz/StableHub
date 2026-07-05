use super::InventoryDomainError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StockLevel(f64);

impl StockLevel {
    pub fn new(value: f64) -> Result<Self, InventoryDomainError> {
        if value < 0.0 {
            return Err(InventoryDomainError::NegativeStock);
        }

        Ok(Self(value))
    }

    pub fn value(self) -> f64 {
        self.0
    }
}
