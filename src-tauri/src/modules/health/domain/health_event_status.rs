#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthEventStatus {
    Active,
    Archived,
}

impl HealthEventStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Archived => "archived",
        }
    }
}

impl TryFrom<&str> for HealthEventStatus {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "active" => Ok(Self::Active),
            "archived" => Ok(Self::Archived),
            other => Err(format!(
                "Nieobsługiwany status zdarzenia zdrowotnego: {other}"
            )),
        }
    }
}
