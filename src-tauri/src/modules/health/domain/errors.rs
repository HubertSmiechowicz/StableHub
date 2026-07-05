#[derive(Debug)]
pub enum HealthDomainError {
    EmptyHorseId,
    EmptyTitle,
    TitleTooLong,
    EmptyDate,
    UnsupportedType(String),
}

impl std::fmt::Display for HealthDomainError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyHorseId => write!(formatter, "Koń jest wymagany"),
            Self::EmptyTitle => write!(formatter, "Tytuł zdarzenia zdrowotnego jest wymagany"),
            Self::TitleTooLong => write!(
                formatter,
                "Tytuł zdarzenia zdrowotnego może mieć maksymalnie 120 znaków"
            ),
            Self::EmptyDate => write!(formatter, "Data zdarzenia zdrowotnego jest wymagana"),
            Self::UnsupportedType(event_type) => write!(
                formatter,
                "Nieobsługiwany typ zdarzenia zdrowotnego: {event_type}"
            ),
        }
    }
}

impl std::error::Error for HealthDomainError {}
