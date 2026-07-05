use crate::modules::horse::{application::ports::HorseRepository, domain::HorseStatus};

pub struct ArchiveHorseCommand {
    pub id: String,
}

pub struct ArchiveHorseHandler<'a, R>
where
    R: HorseRepository,
{
    repository: &'a R,
}

impl<'a, R> ArchiveHorseHandler<'a, R>
where
    R: HorseRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, command: ArchiveHorseCommand) -> Result<(), String> {
        let mut profile = self
            .repository
            .find_profile_by_id(&command.id)
            .await?
            .ok_or_else(|| "Nie znaleziono konia".to_string())?;

        if profile.horse.status() == HorseStatus::Archived {
            return Ok(());
        }

        let now = current_timestamp();
        profile.horse.archive();
        profile.updated_at = now.clone();
        profile.archived_at = Some(now);

        self.repository.save_profile(&profile).await
    }
}

fn current_timestamp() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}
