use crate::modules::horse::domain::Horse;

#[derive(Debug, Clone)]
pub struct HorseProfileData {
    pub horse: Horse,
    pub sex: Option<String>,
    pub breed: Option<String>,
    pub date_of_birth: Option<String>,
    pub coat_color: Option<String>,
    pub identification_number: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
}
