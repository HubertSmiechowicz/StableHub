use super::HealthDomainError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HorseReference(String);

impl HorseReference {
    pub fn new(value: String) -> Result<Self, HealthDomainError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(HealthDomainError::EmptyHorseId);
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
