use super::HealthDomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthEventType {
    Vaccination,
    Deworming,
    VetVisit,
    Farrier,
    Treatment,
    Checkup,
    Other,
}

impl HealthEventType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Vaccination => "vaccination",
            Self::Deworming => "deworming",
            Self::VetVisit => "vet_visit",
            Self::Farrier => "farrier",
            Self::Treatment => "treatment",
            Self::Checkup => "checkup",
            Self::Other => "other",
        }
    }
}

impl TryFrom<&str> for HealthEventType {
    type Error = HealthDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "vaccination" => Ok(Self::Vaccination),
            "deworming" => Ok(Self::Deworming),
            "vet_visit" => Ok(Self::VetVisit),
            "farrier" => Ok(Self::Farrier),
            "treatment" => Ok(Self::Treatment),
            "checkup" => Ok(Self::Checkup),
            "other" => Ok(Self::Other),
            other => Err(HealthDomainError::UnsupportedType(other.to_string())),
        }
    }
}
