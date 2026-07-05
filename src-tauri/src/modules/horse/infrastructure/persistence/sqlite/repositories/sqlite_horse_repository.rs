use async_trait::async_trait;
use sqlx::{FromRow, SqlitePool};

use crate::modules::horse::{
    application::ports::HorseRepository,
    domain::{Horse, HorseId, HorseProfile, HorseStatus, Sex},
};

pub struct SqliteHorseRepository {
    pool: SqlitePool,
}

impl SqliteHorseRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HorseRepository for SqliteHorseRepository {
    async fn save(&self, horse: &Horse) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO horses (
                id,
                name,
                sex,
                breed,
                date_of_birth,
                coat_color,
                identification_number,
                notes,
                status,
                created_at,
                updated_at,
                archived_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                sex = excluded.sex,
                breed = excluded.breed,
                date_of_birth = excluded.date_of_birth,
                coat_color = excluded.coat_color,
                identification_number = excluded.identification_number,
                notes = excluded.notes,
                status = excluded.status,
                updated_at = excluded.updated_at,
                archived_at = excluded.archived_at
            "#,
        )
        .bind(horse.id().as_str())
        .bind(horse.profile().name())
        .bind(horse.profile().sex().map(|sex| sex.as_str().to_string()))
        .bind(horse.profile().breed())
        .bind(horse.profile().date_of_birth())
        .bind(horse.profile().coat_color())
        .bind(horse.profile().identification_number())
        .bind(horse.profile().notes())
        .bind(horse.status().as_str())
        .bind(horse.created_at())
        .bind(horse.updated_at())
        .bind(horse.archived_at())
        .execute(&self.pool)
        .await
        .map_err(|error| format!("Could not save horse: {error}"))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Horse>, String> {
        let row = sqlx::query_as::<_, HorseRow>(
            r#"
            SELECT
                id,
                name,
                sex,
                breed,
                date_of_birth,
                coat_color,
                identification_number,
                notes,
                status,
                created_at,
                updated_at,
                archived_at
            FROM horses
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| format!("Could not find horse: {error}"))?;

        row.map(HorseRow::try_into).transpose()
    }

    async fn list_active(&self) -> Result<Vec<Horse>, String> {
        let rows = sqlx::query_as::<_, HorseRow>(
            r#"
            SELECT
                id,
                name,
                sex,
                breed,
                date_of_birth,
                coat_color,
                identification_number,
                notes,
                status,
                created_at,
                updated_at,
                archived_at
            FROM horses
            WHERE status = 'active'
            ORDER BY name COLLATE NOCASE ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|error| format!("Could not list horses: {error}"))?;

        rows.into_iter().map(HorseRow::try_into).collect()
    }
}

#[derive(Debug, FromRow)]
struct HorseRow {
    id: String,
    name: String,
    sex: Option<String>,
    breed: Option<String>,
    date_of_birth: Option<String>,
    coat_color: Option<String>,
    identification_number: Option<String>,
    notes: Option<String>,
    status: String,
    created_at: String,
    updated_at: String,
    archived_at: Option<String>,
}

impl TryFrom<HorseRow> for Horse {
    type Error = String;

    fn try_from(row: HorseRow) -> Result<Self, Self::Error> {
        let sex = row.sex.as_deref().map(Sex::try_from).transpose()?;
        let profile = HorseProfile::new(
            row.name,
            sex,
            row.breed,
            row.date_of_birth,
            row.coat_color,
            row.identification_number,
            row.notes,
        )
        .map_err(|error| error.to_string())?;

        let status = HorseStatus::try_from(row.status.as_str())?;

        Ok(Horse::restore(
            HorseId::new(row.id),
            profile,
            status,
            row.created_at,
            row.updated_at,
            row.archived_at,
        ))
    }
}
