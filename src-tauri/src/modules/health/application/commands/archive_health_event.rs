use crate::modules::health::{
    application::ports::HealthEventRepository, domain::HealthEventStatus,
};

pub struct ArchiveHealthEventCommand {
    pub id: String,
}

pub struct ArchiveHealthEventHandler<'a, R>
where
    R: HealthEventRepository,
{
    repository: &'a R,
}

impl<'a, R> ArchiveHealthEventHandler<'a, R>
where
    R: HealthEventRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, command: ArchiveHealthEventCommand) -> Result<(), String> {
        let mut profile = self
            .repository
            .find_profile_by_id(&command.id)
            .await?
            .ok_or_else(|| "Nie znaleziono zdarzenia zdrowotnego".to_string())?;

        if profile.event.status() == HealthEventStatus::Archived {
            return Ok(());
        }

        let now = current_timestamp();
        profile.event.archive();
        profile.updated_at = now.clone();
        profile.archived_at = Some(now);

        self.repository.save_event(&profile).await
    }
}

fn current_timestamp() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}
