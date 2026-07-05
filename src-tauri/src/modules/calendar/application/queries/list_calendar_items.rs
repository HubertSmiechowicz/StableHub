use crate::modules::calendar::application::{
    dto::CalendarItemSummary,
    ports::{CalendarItemSortBy, CalendarListOptions, CalendarRepository, SortDirection},
};

pub struct ListCalendarItemsQuery {
    pub search: Option<String>,
    pub source_module: Option<String>,
    pub item_type: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub sort_by: String,
    pub sort_direction: String,
}

pub struct ListCalendarItemsHandler<'a, R>
where
    R: CalendarRepository,
{
    repository: &'a R,
}

impl<'a, R> ListCalendarItemsHandler<'a, R>
where
    R: CalendarRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        query: ListCalendarItemsQuery,
    ) -> Result<Vec<CalendarItemSummary>, String> {
        let options = CalendarListOptions {
            search: normalize(query.search),
            source_module: normalize(query.source_module),
            item_type: normalize(query.item_type),
            date_from: normalize(query.date_from),
            date_to: normalize(query.date_to),
            sort_by: match query.sort_by.as_str() {
                "title" => CalendarItemSortBy::Title,
                "type" => CalendarItemSortBy::Type,
                "source" => CalendarItemSortBy::Source,
                _ => CalendarItemSortBy::ScheduledOn,
            },
            sort_direction: match query.sort_direction.as_str() {
                "asc" => SortDirection::Asc,
                _ => SortDirection::Desc,
            },
        };

        self.repository.list_items(&options).await
    }
}

fn normalize(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}
