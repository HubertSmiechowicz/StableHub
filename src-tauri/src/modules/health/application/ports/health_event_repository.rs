use async_trait::async_trait;

use crate::modules::health::application::dto::{
    HealthEventDetails, HealthEventProfileData, HealthEventSummary,
};

#[derive(Debug, Clone)]
pub struct HealthListOptions {
    pub search: Option<String>,
    pub horse_id: Option<String>,
    pub event_type: Option<String>,
    pub sort_by: HealthEventSortBy,
    pub sort_direction: SortDirection,
}

#[derive(Debug, Clone, Copy)]
pub enum HealthEventSortBy {
    OccurredOn,
    Title,
    Type,
    Horse,
    CreatedAt,
}

#[derive(Debug, Clone, Copy)]
pub enum SortDirection {
    Asc,
    Desc,
}

#[async_trait]
pub trait HealthEventRepository {
    async fn save_event(&self, profile: &HealthEventProfileData) -> Result<(), String>;
    async fn find_profile_by_id(&self, id: &str) -> Result<Option<HealthEventProfileData>, String>;
    async fn find_details_by_id(&self, id: &str) -> Result<Option<HealthEventDetails>, String>;
    async fn list_active_summaries(
        &self,
        options: &HealthListOptions,
    ) -> Result<Vec<HealthEventSummary>, String>;
    async fn active_horse_exists(&self, horse_id: &str) -> Result<bool, String>;
}
