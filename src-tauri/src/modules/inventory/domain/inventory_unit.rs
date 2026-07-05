use super::InventoryDomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InventoryUnit {
    Piece,
    Kilogram,
    Bale,
    Liter,
    Bag,
    Package,
}

impl InventoryUnit {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Piece => "piece",
            Self::Kilogram => "kg",
            Self::Bale => "bale",
            Self::Liter => "liter",
            Self::Bag => "bag",
            Self::Package => "package",
        }
    }
}

impl TryFrom<&str> for InventoryUnit {
    type Error = InventoryDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "piece" => Ok(Self::Piece),
            "kg" => Ok(Self::Kilogram),
            "bale" => Ok(Self::Bale),
            "liter" => Ok(Self::Liter),
            "bag" => Ok(Self::Bag),
            "package" => Ok(Self::Package),
            other => Err(InventoryDomainError::UnsupportedUnit(other.to_string())),
        }
    }
}
