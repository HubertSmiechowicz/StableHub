#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InventoryItemStatus {
    Active,
    Archived,
}

impl InventoryItemStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Archived => "archived",
        }
    }
}

impl TryFrom<&str> for InventoryItemStatus {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "active" => Ok(Self::Active),
            "archived" => Ok(Self::Archived),
            other => Err(format!(
                "Nieobsługiwany status pozycji magazynowej: {other}"
            )),
        }
    }
}
