use super::InventoryDomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventoryItemName(String);

impl InventoryItemName {
    pub fn new(value: String) -> Result<Self, InventoryDomainError> {
        let value = value.trim().to_string();

        if value.is_empty() {
            return Err(InventoryDomainError::EmptyName);
        }

        if value.chars().count() > 100 {
            return Err(InventoryDomainError::NameTooLong);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
