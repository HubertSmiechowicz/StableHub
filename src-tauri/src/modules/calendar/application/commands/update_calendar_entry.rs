use crate::modules::calendar::{
    application::{dto::CalendarEntryDetails, ports::CalendarRepository},
    domain::{CalendarEntryDate, CalendarEntryStatus, CalendarEntryTitle, CalendarEntryType},
};

pub struct UpdateCalendarEntryCommand {
    pub id: String,
    pub title: String,
    pub scheduled_on: String,
    pub scheduled_time: Option<String>,
    pub entry_type: String,
    pub description: Option<String>,
    pub status: String,
}

pub struct UpdateCalendarEntryHandler<'a, R>
where
    R: CalendarRepository,
{
    repository: &'a R,
}

impl<'a, R> UpdateCalendarEntryHandler<'a, R>
where
    R: CalendarRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: UpdateCalendarEntryCommand,
    ) -> Result<CalendarEntryDetails, String> {
        let mut profile = self
            .repository
            .find_profile_by_id(&command.id)
            .await?
            .ok_or_else(|| "Nie znaleziono wpisu kalendarza".to_string())?;

        if profile.entry.status() == CalendarEntryStatus::Archived {
            return Err("Nie można edytować usuniętego wpisu kalendarza".to_string());
        }

        profile.entry.update(
            CalendarEntryTitle::new(command.title).map_err(|error| error.to_string())?,
            CalendarEntryDate::new(command.scheduled_on).map_err(|error| error.to_string())?,
            CalendarEntryType::try_from(command.entry_type.as_str())
                .map_err(|error| error.to_string())?,
            CalendarEntryStatus::try_from(command.status.as_str())?,
        );
        profile.scheduled_time = normalize_optional(command.scheduled_time);
        profile.description = normalize_optional(command.description);
        profile.updated_at = current_timestamp();

        self.repository.save_entry(&profile).await?;

        self.repository
            .find_details_by_id(profile.entry.id().as_str())
            .await?
            .ok_or_else(|| "Nie znaleziono wpisu kalendarza po aktualizacji".to_string())
    }
}

fn normalize_optional(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn current_timestamp() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}
