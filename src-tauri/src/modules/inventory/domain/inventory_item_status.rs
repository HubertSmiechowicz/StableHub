#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InventoryItemStatus {
    Active,
}

impl InventoryItemStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
        }
    }
}
