#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HorseId(String);

impl HorseId {
    pub fn generate() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

}
