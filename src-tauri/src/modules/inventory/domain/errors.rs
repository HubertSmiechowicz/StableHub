#[derive(Debug, Clone, PartialEq)]
pub enum InventoryDomainError {
    EmptyName,
    NameTooLong,
    NegativeStock,
    UnsupportedUnit(String),
}

impl std::fmt::Display for InventoryDomainError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyName => write!(formatter, "Nazwa pozycji magazynowej jest wymagana"),
            Self::NameTooLong => {
                write!(formatter, "Nazwa pozycji magazynowej nie może być dłuższa niż 100 znaków")
            }
            Self::NegativeStock => write!(formatter, "Stan magazynowy nie może być ujemny"),
            Self::UnsupportedUnit(unit) => write!(formatter, "Nieobsługiwana jednostka magazynowa: {unit}"),
        }
    }
}

impl std::error::Error for InventoryDomainError {}
