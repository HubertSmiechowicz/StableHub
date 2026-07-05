use crate::modules::horse::application::{
    dto::HorseSummary,
    ports::{HorseListOptions, HorseRepository, HorseSortBy, SortDirection},
};

pub struct ListHorsesQuery {
    pub search: Option<String>,
    pub sort_by: String,
    pub sort_direction: String,
}

pub struct ListHorsesHandler<'a, R>
where
    R: HorseRepository,
{
    repository: &'a R,
}

impl<'a, R> ListHorsesHandler<'a, R>
where
    R: HorseRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, query: ListHorsesQuery) -> Result<Vec<HorseSummary>, String> {
        let options = HorseListOptions {
            search: normalize_search(query.search),
            sort_by: match query.sort_by.as_str() {
                "breed" => HorseSortBy::Breed,
                "created_at" => HorseSortBy::CreatedAt,
                _ => HorseSortBy::Name,
            },
            sort_direction: match query.sort_direction.as_str() {
                "desc" => SortDirection::Desc,
                _ => SortDirection::Asc,
            },
        };

        self.repository.list_active_summaries(&options).await
    }
}

fn normalize_search(search: Option<String>) -> Option<String> {
    search
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}
