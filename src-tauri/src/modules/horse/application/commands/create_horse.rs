use crate::modules::horse::{
    application::{
        dto::{HorseDetails, HorseProfileData},
        ports::HorseRepository,
    },
    domain::{Horse, HorseId, HorseName},
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
        let name = HorseName::new(command.name).map_err(|error| error.to_string())?;
        let now = current_timestamp();
        let horse = Horse::create(HorseId::generate(), name);
        let profile = HorseProfileData {
            horse,
            sex: normalize_optional_text(command.sex),
            breed: normalize_optional_text(command.breed),
            date_of_birth: normalize_optional_text(command.date_of_birth),
            coat_color: normalize_optional_text(command.coat_color),
            identification_number: normalize_optional_text(command.identification_number),
            notes: normalize_optional_text(command.notes),
            created_at: now.clone(),
            updated_at: now,
            archived_at: None,
        };

        self.repository.save_profile(&profile).await?;

        Ok(HorseDetails {
            id: profile.horse.id().as_str().to_string(),
            name: profile.horse.name().as_str().to_string(),
            sex: profile.sex,
            breed: profile.breed,
            date_of_birth: profile.date_of_birth,
            coat_color: profile.coat_color,
            identification_number: profile.identification_number,
            notes: profile.notes,
            status: profile.horse.status().as_str().to_string(),
            created_at: profile.created_at,
            updated_at: profile.updated_at,
            archived_at: profile.archived_at,
        })
    }
}

fn current_timestamp() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value
        .map(|text| text.trim().to_string())
        .filter(|text| !text.is_empty())
}
