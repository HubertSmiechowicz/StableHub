#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HorseDomainError {
    EmptyName,
    NameTooLong,
}

impl std::fmt::Display for HorseDomainError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyName => write!(formatter, "Horse name is required"),
            Self::NameTooLong => write!(formatter, "Horse name cannot be longer than 80 characters"),
        }
    }
}

impl std::error::Error for HorseDomainError {}
