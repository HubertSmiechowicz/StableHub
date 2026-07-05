use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CalendarItemSummary {
    pub id: String,
    pub source_module: String,
    pub source_id: String,
    pub title: String,
    pub scheduled_on: String,
    pub scheduled_time: Option<String>,
    pub item_type: String,
    pub description: Option<String>,
    pub status: String,
    pub related_label: Option<String>,
}
