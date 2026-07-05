use crate::modules::horse::application::{dto::HorseSummary, ports::HorseRepository};

pub struct ListHorsesQuery;

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

    pub async fn handle(&self, _query: ListHorsesQuery) -> Result<Vec<HorseSummary>, String> {
        let horses = self.repository.list_active().await?;
        Ok(horses.into_iter().map(HorseSummary::from).collect())
    }
}
