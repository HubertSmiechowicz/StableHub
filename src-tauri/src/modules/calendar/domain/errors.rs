#[derive(Debug)]
pub enum CalendarDomainError {
    EmptyTitle,
    TitleTooLong,
    EmptyDate,
    UnsupportedType(String),
}

impl std::fmt::Display for CalendarDomainError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyTitle => write!(formatter, "Tytuł wpisu kalendarza jest wymagany"),
            Self::TitleTooLong => {
                write!(
                    formatter,
                    "Tytuł wpisu kalendarza może mieć maksymalnie 120 znaków"
                )
            }
            Self::EmptyDate => write!(formatter, "Data wpisu kalendarza jest wymagana"),
            Self::UnsupportedType(entry_type) => {
                write!(
                    formatter,
                    "Nieobsługiwany typ wpisu kalendarza: {entry_type}"
                )
            }
        }
    }
}

impl std::error::Error for CalendarDomainError {}
