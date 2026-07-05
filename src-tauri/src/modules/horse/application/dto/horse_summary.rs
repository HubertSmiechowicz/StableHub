use serde::Serialize;

use crate::modules::horse::domain::Horse;

#[derive(Debug, Serialize)]
pub struct HorseSummary {
    pub id: String,
    pub name: String,
    pub sex: Option<String>,
    pub breed: Option<String>,
    pub status: String,
}

impl From<Horse> for HorseSummary {
    fn from(horse: Horse) -> Self {
        Self {
            id: horse.id().as_str().to_string(),
            name: horse.profile().name().to_string(),
            sex: horse.profile().sex().map(|sex| sex.as_str().to_string()),
            breed: horse.profile().breed().map(ToOwned::to_owned),
            status: horse.status().as_str().to_string(),
        }
    }
}
