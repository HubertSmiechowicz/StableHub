use super::HealthDomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealthEventTitle(String);

impl HealthEventTitle {
    pub fn new(value: String) -> Result<Self, HealthDomainError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(HealthDomainError::EmptyTitle);
        }

        if trimmed.chars().count() > 120 {
            return Err(HealthDomainError::TitleTooLong);
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
