use super::CalendarDomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalendarEntryType {
    Task,
    Reminder,
    Visit,
    Payment,
    Other,
}

impl CalendarEntryType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Task => "task",
            Self::Reminder => "reminder",
            Self::Visit => "visit",
            Self::Payment => "payment",
            Self::Other => "other",
        }
    }
}

impl TryFrom<&str> for CalendarEntryType {
    type Error = CalendarDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "task" => Ok(Self::Task),
            "reminder" => Ok(Self::Reminder),
            "visit" => Ok(Self::Visit),
            "payment" => Ok(Self::Payment),
            "other" => Ok(Self::Other),
            other => Err(CalendarDomainError::UnsupportedType(other.to_string())),
        }
    }
}
