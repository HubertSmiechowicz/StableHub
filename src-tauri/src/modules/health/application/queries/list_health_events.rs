use crate::modules::health::application::{
    dto::HealthEventSummary,
    ports::{HealthEventRepository, HealthEventSortBy, HealthListOptions, SortDirection},
};

pub struct ListHealthEventsQuery {
    pub search: Option<String>,
    pub horse_id: Option<String>,
    pub event_type: Option<String>,
    pub sort_by: String,
    pub sort_direction: String,
}

pub struct ListHealthEventsHandler<'a, R>
where
    R: HealthEventRepository,
{
    repository: &'a R,
}

impl<'a, R> ListHealthEventsHandler<'a, R>
where
    R: HealthEventRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        query: ListHealthEventsQuery,
    ) -> Result<Vec<HealthEventSummary>, String> {
        let options = HealthListOptions {
            search: normalize(query.search),
            horse_id: normalize(query.horse_id),
            event_type: normalize(query.event_type),
            sort_by: match query.sort_by.as_str() {
                "title" => HealthEventSortBy::Title,
                "type" => HealthEventSortBy::Type,
                "horse" => HealthEventSortBy::Horse,
                "created_at" => HealthEventSortBy::CreatedAt,
                _ => HealthEventSortBy::OccurredOn,
            },
            sort_direction: match query.sort_direction.as_str() {
                "asc" => SortDirection::Asc,
                _ => SortDirection::Desc,
            },
        };

        self.repository.list_active_summaries(&options).await
    }
}

fn normalize(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}
