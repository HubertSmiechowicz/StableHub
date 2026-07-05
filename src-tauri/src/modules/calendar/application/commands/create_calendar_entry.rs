use crate::modules::calendar::{
    application::{
        dto::{CalendarEntryDetails, CalendarEntryProfileData},
        ports::CalendarRepository,
    },
    domain::{
        CalendarEntry, CalendarEntryDate, CalendarEntryId, CalendarEntryTitle, CalendarEntryType,
    },
};

pub struct CreateCalendarEntryCommand {
    pub title: String,
    pub scheduled_on: String,
    pub scheduled_time: Option<String>,
    pub entry_type: String,
    pub description: Option<String>,
}

pub struct CreateCalendarEntryHandler<'a, R>
where
    R: CalendarRepository,
{
    repository: &'a R,
}

impl<'a, R> CreateCalendarEntryHandler<'a, R>
where
    R: CalendarRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: CreateCalendarEntryCommand,
    ) -> Result<CalendarEntryDetails, String> {
        let title = CalendarEntryTitle::new(command.title).map_err(|error| error.to_string())?;
        let scheduled_on =
            CalendarEntryDate::new(command.scheduled_on).map_err(|error| error.to_string())?;
        let entry_type = CalendarEntryType::try_from(command.entry_type.as_str())
            .map_err(|error| error.to_string())?;
        let entry =
            CalendarEntry::create(CalendarEntryId::generate(), title, scheduled_on, entry_type);
        let now = current_timestamp();
        let profile = CalendarEntryProfileData {
            entry,
            scheduled_time: normalize_optional(command.scheduled_time),
            description: normalize_optional(command.description),
            created_at: now.clone(),
            updated_at: now,
            archived_at: None,
        };

        self.repository.save_entry(&profile).await?;

        self.repository
            .find_details_by_id(profile.entry.id().as_str())
            .await?
            .ok_or_else(|| "Nie znaleziono wpisu kalendarza po zapisie".to_string())
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
