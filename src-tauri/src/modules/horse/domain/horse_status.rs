#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HorseStatus {
    Active,
    Archived,
}

impl HorseStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Archived => "archived",
        }
    }
}

impl TryFrom<&str> for HorseStatus {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "active" => Ok(Self::Active),
            "archived" => Ok(Self::Archived),
            other => Err(format!("Unsupported horse status: {other}")),
        }
    }
}
