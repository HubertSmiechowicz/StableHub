use async_trait::async_trait;

use crate::modules::horse::application::dto::{HorseDetails, HorseProfileData, HorseSummary};

#[async_trait]
pub trait HorseRepository {
    async fn save_profile(&self, profile: &HorseProfileData) -> Result<(), String>;
    async fn find_details_by_id(&self, id: &str) -> Result<Option<HorseDetails>, String>;
    async fn list_active_summaries(&self) -> Result<Vec<HorseSummary>, String>;
}
