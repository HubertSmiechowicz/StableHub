use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CalendarEntryDetails {
    pub id: String,
    pub title: String,
    pub scheduled_on: String,
    pub scheduled_time: Option<String>,
    pub entry_type: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
}
