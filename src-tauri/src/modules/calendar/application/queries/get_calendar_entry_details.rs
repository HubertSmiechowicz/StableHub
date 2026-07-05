use crate::modules::calendar::application::{dto::CalendarEntryDetails, ports::CalendarRepository};

pub struct GetCalendarEntryDetailsQuery {
    pub id: String,
}

pub struct GetCalendarEntryDetailsHandler<'a, R>
where
    R: CalendarRepository,
{
    repository: &'a R,
}

impl<'a, R> GetCalendarEntryDetailsHandler<'a, R>
where
    R: CalendarRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        query: GetCalendarEntryDetailsQuery,
    ) -> Result<CalendarEntryDetails, String> {
        self.repository
            .find_details_by_id(&query.id)
            .await?
            .ok_or_else(|| "Nie znaleziono wpisu kalendarza".to_string())
    }
}
