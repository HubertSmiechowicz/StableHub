use async_trait::async_trait;
use sqlx::{FromRow, SqlitePool};

use crate::modules::horse::application::{
    dto::{HorseDetails, HorseProfileData, HorseSummary},
    ports::HorseRepository,
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
    async fn save_profile(&self, profile: &HorseProfileData) -> Result<(), String> {
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
        .bind(profile.horse.id().as_str())
        .bind(profile.horse.name().as_str())
        .bind(profile.sex.as_deref())
        .bind(profile.breed.as_deref())
        .bind(profile.date_of_birth.as_deref())
        .bind(profile.coat_color.as_deref())
        .bind(profile.identification_number.as_deref())
        .bind(profile.notes.as_deref())
        .bind(profile.horse.status().as_str())
        .bind(profile.created_at.as_str())
        .bind(profile.updated_at.as_str())
        .bind(profile.archived_at.as_deref())
        .execute(&self.pool)
        .await
        .map_err(|error| format!("Could not save horse: {error}"))?;

        Ok(())
    }

    async fn find_details_by_id(&self, id: &str) -> Result<Option<HorseDetails>, String> {
        let row = sqlx::query_as::<_, HorseDetailsRow>(
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

        Ok(row.map(HorseDetails::from))
    }

    async fn list_active_summaries(&self) -> Result<Vec<HorseSummary>, String> {
        let rows = sqlx::query_as::<_, HorseSummaryRow>(
            r#"
            SELECT
                id,
                name,
                sex,
                breed,
                status
            FROM horses
            WHERE status = 'active'
            ORDER BY name COLLATE NOCASE ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|error| format!("Could not list horses: {error}"))?;

        Ok(rows.into_iter().map(HorseSummary::from).collect())
    }
}

#[derive(Debug, FromRow)]
struct HorseSummaryRow {
    id: String,
    name: String,
    sex: Option<String>,
    breed: Option<String>,
    status: String,
}

impl From<HorseSummaryRow> for HorseSummary {
    fn from(row: HorseSummaryRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
            sex: row.sex,
            breed: row.breed,
            status: row.status,
        }
    }
}

#[derive(Debug, FromRow)]
struct HorseDetailsRow {
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

impl From<HorseDetailsRow> for HorseDetails {
    fn from(row: HorseDetailsRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
            sex: row.sex,
            breed: row.breed,
            date_of_birth: row.date_of_birth,
            coat_color: row.coat_color,
            identification_number: row.identification_number,
            notes: row.notes,
            status: row.status,
            created_at: row.created_at,
            updated_at: row.updated_at,
            archived_at: row.archived_at,
        }
    }
}
