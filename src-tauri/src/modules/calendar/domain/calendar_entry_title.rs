use super::CalendarDomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalendarEntryTitle(String);

impl CalendarEntryTitle {
    pub fn new(value: String) -> Result<Self, CalendarDomainError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(CalendarDomainError::EmptyTitle);
        }

        if trimmed.chars().count() > 120 {
            return Err(CalendarDomainError::TitleTooLong);
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
