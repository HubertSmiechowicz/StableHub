use crate::modules::health::{
    application::{dto::HealthEventDetails, ports::HealthEventRepository},
    domain::{
        HealthEventDate, HealthEventStatus, HealthEventTitle, HealthEventType, HorseReference,
    },
};

pub struct UpdateHealthEventCommand {
    pub id: String,
    pub horse_id: String,
    pub event_type: String,
    pub occurred_on: String,
    pub title: String,
    pub notes: Option<String>,
    pub cost: Option<f64>,
}

pub struct UpdateHealthEventHandler<'a, R>
where
    R: HealthEventRepository,
{
    repository: &'a R,
}

impl<'a, R> UpdateHealthEventHandler<'a, R>
where
    R: HealthEventRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        command: UpdateHealthEventCommand,
    ) -> Result<HealthEventDetails, String> {
        let mut profile = self
            .repository
            .find_profile_by_id(&command.id)
            .await?
            .ok_or_else(|| "Nie znaleziono zdarzenia zdrowotnego".to_string())?;

        if profile.event.status() == HealthEventStatus::Archived {
            return Err("Nie można edytować usuniętego zdarzenia zdrowotnego".to_string());
        }

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

        profile
            .event
            .update(horse_id, event_type, occurred_on, title);
        profile.notes = command.notes;
        profile.cost = normalize_non_negative(command.cost)?;
        profile.updated_at = current_timestamp();

        self.repository.save_event(&profile).await?;

        self.repository
            .find_details_by_id(profile.event.id().as_str())
            .await?
            .ok_or_else(|| "Nie znaleziono zdarzenia zdrowotnego po aktualizacji".to_string())
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
