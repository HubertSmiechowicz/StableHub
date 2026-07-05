use crate::modules::horse::{
    application::{dto::HorseDetails, ports::HorseRepository},
    domain::HorseName,
};

pub struct UpdateHorseCommand {
    pub id: String,
    pub name: String,
    pub sex: Option<String>,
    pub breed: Option<String>,
    pub date_of_birth: Option<String>,
    pub coat_color: Option<String>,
    pub identification_number: Option<String>,
    pub notes: Option<String>,
}

pub struct UpdateHorseHandler<'a, R>
where
    R: HorseRepository,
{
    repository: &'a R,
}

impl<'a, R> UpdateHorseHandler<'a, R>
where
    R: HorseRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, command: UpdateHorseCommand) -> Result<HorseDetails, String> {
        let mut profile = self
            .repository
            .find_profile_by_id(&command.id)
            .await?
            .ok_or_else(|| "Nie znaleziono konia".to_string())?;

        if profile.archived_at.is_some() {
            return Err("Nie można edytować usuniętego konia".to_string());
        }

        profile
            .horse
            .rename(HorseName::new(command.name).map_err(|error| error.to_string())?);
        profile.sex = command.sex;
        profile.breed = command.breed;
        profile.date_of_birth = command.date_of_birth;
        profile.coat_color = command.coat_color;
        profile.identification_number = command.identification_number;
        profile.notes = command.notes;
        profile.updated_at = current_timestamp();

        self.repository.save_profile(&profile).await?;

        self.repository
            .find_details_by_id(profile.horse.id().as_str())
            .await?
            .ok_or_else(|| "Nie znaleziono konia po aktualizacji".to_string())
    }
}

fn current_timestamp() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}
