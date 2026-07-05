use serde::Serialize;

use crate::modules::horse::domain::Horse;

#[derive(Debug, Serialize)]
pub struct HorseDetails {
    pub id: String,
    pub name: String,
    pub sex: Option<String>,
    pub breed: Option<String>,
    pub date_of_birth: Option<String>,
    pub coat_color: Option<String>,
    pub identification_number: Option<String>,
    pub notes: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
}

impl From<Horse> for HorseDetails {
    fn from(horse: Horse) -> Self {
        Self {
            id: horse.id().as_str().to_string(),
            name: horse.profile().name().to_string(),
            sex: horse.profile().sex().map(|sex| sex.as_str().to_string()),
            breed: horse.profile().breed().map(ToOwned::to_owned),
            date_of_birth: horse.profile().date_of_birth().map(ToOwned::to_owned),
            coat_color: horse.profile().coat_color().map(ToOwned::to_owned),
            identification_number: horse
                .profile()
                .identification_number()
                .map(ToOwned::to_owned),
            notes: horse.profile().notes().map(ToOwned::to_owned),
            status: horse.status().as_str().to_string(),
            created_at: horse.created_at().to_string(),
            updated_at: horse.updated_at().to_string(),
            archived_at: horse.archived_at().map(ToOwned::to_owned),
        }
    }
}
