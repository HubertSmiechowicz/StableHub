use super::CalendarDomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalendarEntryDate(String);

impl CalendarEntryDate {
    pub fn new(value: String) -> Result<Self, CalendarDomainError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(CalendarDomainError::EmptyDate);
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
