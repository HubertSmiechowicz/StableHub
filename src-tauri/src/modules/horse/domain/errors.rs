#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HorseDomainError {
    EmptyName,
    NameTooLong,
}

impl std::fmt::Display for HorseDomainError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyName => write!(formatter, "Imię konia jest wymagane"),
            Self::NameTooLong => write!(formatter, "Imię konia nie może być dłuższe niż 80 znaków"),
        }
    }
}

impl std::error::Error for HorseDomainError {}
