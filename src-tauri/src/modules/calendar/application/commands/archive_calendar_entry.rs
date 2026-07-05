use crate::modules::calendar::{
    application::ports::CalendarRepository, domain::CalendarEntryStatus,
};

pub struct ArchiveCalendarEntryCommand {
    pub id: String,
}

pub struct ArchiveCalendarEntryHandler<'a, R>
where
    R: CalendarRepository,
{
    repository: &'a R,
}

impl<'a, R> ArchiveCalendarEntryHandler<'a, R>
where
    R: CalendarRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, command: ArchiveCalendarEntryCommand) -> Result<(), String> {
        let mut profile = self
            .repository
            .find_profile_by_id(&command.id)
            .await?
            .ok_or_else(|| "Nie znaleziono wpisu kalendarza".to_string())?;

        if profile.entry.status() == CalendarEntryStatus::Archived {
            return Ok(());
        }

        let now = current_timestamp();
        profile.entry.archive();
        profile.updated_at = now.clone();
        profile.archived_at = Some(now);

        self.repository.save_entry(&profile).await
    }
}

fn current_timestamp() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}
