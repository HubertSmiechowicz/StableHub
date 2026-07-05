#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CalendarEntryId(String);

impl CalendarEntryId {
    pub fn generate() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }

    pub fn from_string(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
