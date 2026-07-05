use super::HealthDomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealthEventDate(String);

impl HealthEventDate {
    pub fn new(value: String) -> Result<Self, HealthDomainError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(HealthDomainError::EmptyDate);
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
