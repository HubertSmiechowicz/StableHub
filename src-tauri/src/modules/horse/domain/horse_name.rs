use super::HorseDomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HorseName(String);

impl HorseName {
    pub fn new(value: String) -> Result<Self, HorseDomainError> {
        let value = value.trim().to_string();

        if value.is_empty() {
            return Err(HorseDomainError::EmptyName);
        }

        if value.chars().count() > 80 {
            return Err(HorseDomainError::NameTooLong);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
