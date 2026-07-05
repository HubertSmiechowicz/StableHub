#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalendarEntryStatus {
    Planned,
    Done,
    Archived,
}

impl CalendarEntryStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::Done => "done",
            Self::Archived => "archived",
        }
    }
}

impl TryFrom<&str> for CalendarEntryStatus {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "planned" => Ok(Self::Planned),
            "done" => Ok(Self::Done),
            "archived" => Ok(Self::Archived),
            other => Err(format!("Nieobsługiwany status wpisu kalendarza: {other}")),
        }
    }
}
