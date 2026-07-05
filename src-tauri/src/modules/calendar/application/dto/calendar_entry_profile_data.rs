use crate::modules::calendar::domain::CalendarEntry;

#[derive(Debug, Clone)]
pub struct CalendarEntryProfileData {
    pub entry: CalendarEntry,
    pub scheduled_time: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
}
