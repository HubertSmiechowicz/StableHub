use super::{HorseId, HorseProfile, HorseStatus};

#[derive(Debug, Clone)]
pub struct Horse {
    id: HorseId,
    profile: HorseProfile,
    status: HorseStatus,
    created_at: String,
    updated_at: String,
    archived_at: Option<String>,
}

impl Horse {
    pub fn create(id: HorseId, profile: HorseProfile, now: String) -> Self {
        Self {
            id,
            profile,
            status: HorseStatus::Active,
            created_at: now.clone(),
            updated_at: now,
            archived_at: None,
        }
    }

    pub fn restore(
        id: HorseId,
        profile: HorseProfile,
        status: HorseStatus,
        created_at: String,
        updated_at: String,
        archived_at: Option<String>,
    ) -> Self {
        Self {
            id,
            profile,
            status,
            created_at,
            updated_at,
            archived_at,
        }
    }

    pub fn id(&self) -> &HorseId {
        &self.id
    }

    pub fn profile(&self) -> &HorseProfile {
        &self.profile
    }

    pub fn status(&self) -> HorseStatus {
        self.status
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }

    pub fn updated_at(&self) -> &str {
        &self.updated_at
    }

    pub fn archived_at(&self) -> Option<&str> {
        self.archived_at.as_deref()
    }
}
