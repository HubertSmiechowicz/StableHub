use async_trait::async_trait;

use crate::modules::horse::domain::Horse;

#[async_trait]
pub trait HorseRepository {
    async fn save(&self, horse: &Horse) -> Result<(), String>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Horse>, String>;
    async fn list_active(&self) -> Result<Vec<Horse>, String>;
}
