use crate::modules::health::application::{dto::HealthEventDetails, ports::HealthEventRepository};

pub struct GetHealthEventDetailsQuery {
    pub id: String,
}

pub struct GetHealthEventDetailsHandler<'a, R>
where
    R: HealthEventRepository,
{
    repository: &'a R,
}

impl<'a, R> GetHealthEventDetailsHandler<'a, R>
where
    R: HealthEventRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        query: GetHealthEventDetailsQuery,
    ) -> Result<HealthEventDetails, String> {
        self.repository
            .find_details_by_id(&query.id)
            .await?
            .ok_or_else(|| "Nie znaleziono zdarzenia zdrowotnego".to_string())
    }
}
