use super::{HorseId, HorseName, HorseStatus};

#[derive(Debug, Clone)]
pub struct Horse {
    id: HorseId,
    name: HorseName,
    status: HorseStatus,
}

impl Horse {
    pub fn create(id: HorseId, name: HorseName) -> Self {
        Self {
            id,
            name,
            status: HorseStatus::Active,
        }
    }

    pub fn id(&self) -> &HorseId {
        &self.id
    }

    pub fn name(&self) -> &HorseName {
        &self.name
    }

    pub fn status(&self) -> HorseStatus {
        self.status
    }
}
