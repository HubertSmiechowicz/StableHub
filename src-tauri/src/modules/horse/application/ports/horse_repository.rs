use async_trait::async_trait;

use crate::modules::horse::application::dto::{HorseDetails, HorseProfileData, HorseSummary};

#[derive(Debug, Clone)]
pub struct HorseListOptions {
    pub search: Option<String>,
    pub sort_by: HorseSortBy,
    pub sort_direction: SortDirection,
}

#[derive(Debug, Clone, Copy)]
pub enum HorseSortBy {
    Name,
    Breed,
    CreatedAt,
}

#[derive(Debug, Clone, Copy)]
pub enum SortDirection {
    Asc,
    Desc,
}

#[async_trait]
pub trait HorseRepository {
    async fn save_profile(&self, profile: &HorseProfileData) -> Result<(), String>;
    async fn find_profile_by_id(&self, id: &str) -> Result<Option<HorseProfileData>, String>;
    async fn find_details_by_id(&self, id: &str) -> Result<Option<HorseDetails>, String>;
    async fn list_active_summaries(
        &self,
        options: &HorseListOptions,
    ) -> Result<Vec<HorseSummary>, String>;
}
