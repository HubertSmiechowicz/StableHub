#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sex {
    Mare,
    Stallion,
    Gelding,
    Unknown,
}

impl Sex {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Mare => "mare",
            Self::Stallion => "stallion",
            Self::Gelding => "gelding",
            Self::Unknown => "unknown",
        }
    }
}

impl TryFrom<&str> for Sex {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "mare" => Ok(Self::Mare),
            "stallion" => Ok(Self::Stallion),
            "gelding" => Ok(Self::Gelding),
            "unknown" => Ok(Self::Unknown),
            other => Err(format!("Unsupported horse sex: {other}")),
        }
    }
}
