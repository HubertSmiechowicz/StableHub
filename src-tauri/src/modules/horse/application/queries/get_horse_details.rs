use crate::modules::horse::application::{dto::HorseDetails, ports::HorseRepository};

pub struct GetHorseDetailsQuery {
    pub id: String,
}

pub struct GetHorseDetailsHandler<'a, R>
where
    R: HorseRepository,
{
    repository: &'a R,
}

impl<'a, R> GetHorseDetailsHandler<'a, R>
where
    R: HorseRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, query: GetHorseDetailsQuery) -> Result<HorseDetails, String> {
        self
            .repository
            .find_details_by_id(&query.id)
            .await?
            .ok_or_else(|| "Nie znaleziono konia".to_string())
    }
}
