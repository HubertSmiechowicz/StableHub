use crate::modules::health::{
    application::{
        dto::{HealthEventDetails, HealthEventProfileData},
        ports::HealthEventRepository,
    },
    domain::{
        HealthEvent, HealthEventDate, HealthEventId, HealthEventTitle, HealthEventType,
        HorseReference,
    },
};

pub struct CreateHealthEventCommand {
    pub horse_id: String,
    pub event_type: String,
    pub occurred_on: String,
    pub title: String,
    pub notes: Option<String>,
    pub cost: Option<f64>,
}

pub struct CreateHealthEventHandler<'a, R>
where
    R: HealthEventRepository,
{
    repository: &'a R,
}

impl<'a, R> CreateHealthEventHandler<'a, R>
where
    R: HealthEventRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: CreateHealthEventCommand,
    ) -> Result<HealthEventDetails, String> {
        let horse_id = HorseReference::new(command.horse_id).map_err(|error| error.to_string())?;

        if !self
            .repository
            .active_horse_exists(horse_id.as_str())
            .await?
        {
            return Err("Nie znaleziono aktywnego konia dla zdarzenia zdrowotnego".to_string());
        }

        let event_type = HealthEventType::try_from(command.event_type.as_str())
            .map_err(|error| error.to_string())?;
        let occurred_on =
            HealthEventDate::new(command.occurred_on).map_err(|error| error.to_string())?;
        let title = HealthEventTitle::new(command.title).map_err(|error| error.to_string())?;
        let cost = normalize_non_negative(command.cost)?;
        let event = HealthEvent::create(
            HealthEventId::generate(),
            horse_id,
            event_type,
            occurred_on,
            title,
        );
        let now = current_timestamp();
        let profile = HealthEventProfileData {
            event,
            notes: command.notes,
            cost,
            created_at: now.clone(),
            updated_at: now,
            archived_at: None,
        };

        self.repository.save_event(&profile).await?;

        self.repository
            .find_details_by_id(profile.event.id().as_str())
            .await?
            .ok_or_else(|| "Nie znaleziono zdarzenia zdrowotnego po zapisie".to_string())
    }
}

fn normalize_non_negative(value: Option<f64>) -> Result<Option<f64>, String> {
    match value {
        Some(value) if value < 0.0 => Err("Koszt zdarzenia nie może być ujemny".to_string()),
        Some(value) if value == 0.0 => Ok(None),
        other => Ok(other),
    }
}

fn current_timestamp() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}
