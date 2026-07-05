use crate::modules::horse::{
    application::{dto::HorseDetails, ports::HorseRepository},
    domain::{Horse, HorseId, HorseProfile, Sex},
};

pub struct CreateHorseCommand {
    pub name: String,
    pub sex: Option<String>,
    pub breed: Option<String>,
    pub date_of_birth: Option<String>,
    pub coat_color: Option<String>,
    pub identification_number: Option<String>,
    pub notes: Option<String>,
}

pub struct CreateHorseHandler<'a, R>
where
    R: HorseRepository,
{
    repository: &'a R,
}

impl<'a, R> CreateHorseHandler<'a, R>
where
    R: HorseRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, command: CreateHorseCommand) -> Result<HorseDetails, String> {
        let sex = command
            .sex
            .as_deref()
            .map(Sex::try_from)
            .transpose()?;

        let profile = HorseProfile::new(
            command.name,
            sex,
            command.breed,
            command.date_of_birth,
            command.coat_color,
            command.identification_number,
            command.notes,
        )
        .map_err(|error| error.to_string())?;

        let now = current_timestamp();
        let horse = Horse::create(HorseId::generate(), profile, now);

        self.repository.save(&horse).await?;

        Ok(HorseDetails::from(horse))
    }
}

fn current_timestamp() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}
