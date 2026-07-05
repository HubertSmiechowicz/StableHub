use async_trait::async_trait;

use crate::modules::calendar::application::dto::{
    CalendarEntryDetails, CalendarEntryProfileData, CalendarItemSummary,
};

#[derive(Debug, Clone)]
pub struct CalendarListOptions {
    pub search: Option<String>,
    pub source_module: Option<String>,
    pub item_type: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub sort_by: CalendarItemSortBy,
    pub sort_direction: SortDirection,
}

#[derive(Debug, Clone, Copy)]
pub enum CalendarItemSortBy {
    ScheduledOn,
    Title,
    Type,
    Source,
}

#[derive(Debug, Clone, Copy)]
pub enum SortDirection {
    Asc,
    Desc,
}

#[async_trait]
pub trait CalendarRepository {
    async fn save_entry(&self, profile: &CalendarEntryProfileData) -> Result<(), String>;
    async fn find_profile_by_id(
        &self,
        id: &str,
    ) -> Result<Option<CalendarEntryProfileData>, String>;
    async fn find_details_by_id(&self, id: &str) -> Result<Option<CalendarEntryDetails>, String>;
    async fn list_items(
        &self,
        options: &CalendarListOptions,
    ) -> Result<Vec<CalendarItemSummary>, String>;
}
