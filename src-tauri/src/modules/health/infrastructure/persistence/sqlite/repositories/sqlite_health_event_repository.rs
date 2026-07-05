use async_trait::async_trait;
use sqlx::{FromRow, SqlitePool};

use crate::modules::health::{
    application::{
        dto::{HealthEventDetails, HealthEventProfileData, HealthEventSummary},
        ports::{HealthEventRepository, HealthEventSortBy, HealthListOptions, SortDirection},
    },
    domain::{
        HealthEvent, HealthEventDate, HealthEventId, HealthEventStatus, HealthEventTitle,
        HealthEventType, HorseReference,
    },
};

pub struct SqliteHealthEventRepository {
    pool: SqlitePool,
}

impl SqliteHealthEventRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HealthEventRepository for SqliteHealthEventRepository {
    async fn save_event(&self, profile: &HealthEventProfileData) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO health_events (
                id,
                horse_id,
                event_type,
                occurred_on,
                occurred_time,
                title,
                notes,
                cost,
                status,
                created_at,
                updated_at,
                archived_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                horse_id = excluded.horse_id,
                event_type = excluded.event_type,
                occurred_on = excluded.occurred_on,
                occurred_time = excluded.occurred_time,
                title = excluded.title,
                notes = excluded.notes,
                cost = excluded.cost,
                status = excluded.status,
                updated_at = excluded.updated_at,
                archived_at = excluded.archived_at
            "#,
        )
        .bind(profile.event.id().as_str())
        .bind(profile.event.horse_id().as_str())
        .bind(profile.event.event_type().as_str())
        .bind(profile.event.occurred_on().as_str())
        .bind(profile.occurred_time.as_deref())
        .bind(profile.event.title().as_str())
        .bind(profile.notes.as_deref())
        .bind(profile.cost)
        .bind(profile.event.status().as_str())
        .bind(profile.created_at.as_str())
        .bind(profile.updated_at.as_str())
        .bind(profile.archived_at.as_deref())
        .execute(&self.pool)
        .await
        .map_err(|error| format!("Nie udało się zapisać zdarzenia zdrowotnego: {error}"))?;

        Ok(())
    }

    async fn find_profile_by_id(&self, id: &str) -> Result<Option<HealthEventProfileData>, String> {
        let row = sqlx::query_as::<_, HealthEventProfileRow>(
            r#"
            SELECT
                health_events.id,
                health_events.horse_id,
                health_events.event_type,
                health_events.occurred_on,
                health_events.occurred_time,
                health_events.title,
                health_events.notes,
                health_events.cost,
                health_events.status,
                health_events.created_at,
                health_events.updated_at,
                health_events.archived_at
            FROM health_events
            WHERE health_events.id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| format!("Nie udało się pobrać profilu zdarzenia zdrowotnego: {error}"))?;

        row.map(HealthEventProfileData::try_from).transpose()
    }

    async fn find_details_by_id(&self, id: &str) -> Result<Option<HealthEventDetails>, String> {
        let row = sqlx::query_as::<_, HealthEventDetailsRow>(
            r#"
            SELECT
                health_events.id,
                health_events.horse_id,
                horses.name AS horse_name,
                health_events.event_type,
                health_events.occurred_on,
                health_events.occurred_time,
                health_events.title,
                health_events.notes,
                health_events.cost,
                health_events.status,
                health_events.created_at,
                health_events.updated_at,
                health_events.archived_at
            FROM health_events
            LEFT JOIN horses ON horses.id = health_events.horse_id
            WHERE health_events.id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| format!("Nie udało się pobrać zdarzenia zdrowotnego: {error}"))?;

        Ok(row.map(HealthEventDetails::from))
    }

    async fn list_active_summaries(
        &self,
        options: &HealthListOptions,
    ) -> Result<Vec<HealthEventSummary>, String> {
        let order_by = match options.sort_by {
            HealthEventSortBy::OccurredOn => "health_events.occurred_on",
            HealthEventSortBy::Title => "health_events.title COLLATE NOCASE",
            HealthEventSortBy::Type => "health_events.event_type",
            HealthEventSortBy::Horse => "horses.name COLLATE NOCASE",
            HealthEventSortBy::CreatedAt => "health_events.created_at",
        };
        let direction = match options.sort_direction {
            SortDirection::Asc => "ASC",
            SortDirection::Desc => "DESC",
        };
        let sql = format!(
            r#"
            SELECT
                health_events.id,
                health_events.horse_id,
                horses.name AS horse_name,
                health_events.event_type,
                health_events.occurred_on,
                health_events.occurred_time,
                health_events.title,
                health_events.cost,
                health_events.status
            FROM health_events
            LEFT JOIN horses ON horses.id = health_events.horse_id
            WHERE health_events.status = 'active'
              AND (? IS NULL OR lower(health_events.title) LIKE lower(?) OR lower(COALESCE(health_events.notes, '')) LIKE lower(?))
              AND (? IS NULL OR health_events.horse_id = ?)
              AND (? IS NULL OR health_events.event_type = ?)
            ORDER BY {order_by} {direction}, health_events.occurred_on DESC
            "#
        );
        let search = options.search.as_ref().map(|value| format!("%{value}%"));

        let rows = sqlx::query_as::<_, HealthEventSummaryRow>(&sql)
            .bind(search.as_deref())
            .bind(search.as_deref())
            .bind(search.as_deref())
            .bind(options.horse_id.as_deref())
            .bind(options.horse_id.as_deref())
            .bind(options.event_type.as_deref())
            .bind(options.event_type.as_deref())
            .fetch_all(&self.pool)
            .await
            .map_err(|error| format!("Nie udało się pobrać listy zdarzeń zdrowotnych: {error}"))?;

        Ok(rows.into_iter().map(HealthEventSummary::from).collect())
    }

    async fn active_horse_exists(&self, horse_id: &str) -> Result<bool, String> {
        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM horses
            WHERE id = ?
              AND status = 'active'
            "#,
        )
        .bind(horse_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|error| {
            format!("Nie udało się sprawdzić konia dla zdarzenia zdrowotnego: {error}")
        })?;

        Ok(count > 0)
    }
}

#[derive(Debug, FromRow)]
struct HealthEventSummaryRow {
    id: String,
    horse_id: String,
    horse_name: Option<String>,
    event_type: String,
    occurred_on: String,
    occurred_time: Option<String>,
    title: String,
    cost: Option<f64>,
    status: String,
}

impl From<HealthEventSummaryRow> for HealthEventSummary {
    fn from(row: HealthEventSummaryRow) -> Self {
        Self {
            id: row.id,
            horse_id: row.horse_id,
            horse_name: row.horse_name,
            event_type: row.event_type,
            occurred_on: row.occurred_on,
            occurred_time: row.occurred_time,
            title: row.title,
            cost: row.cost,
            status: row.status,
        }
    }
}

#[derive(Debug, FromRow)]
struct HealthEventDetailsRow {
    id: String,
    horse_id: String,
    horse_name: Option<String>,
    event_type: String,
    occurred_on: String,
    occurred_time: Option<String>,
    title: String,
    notes: Option<String>,
    cost: Option<f64>,
    status: String,
    created_at: String,
    updated_at: String,
    archived_at: Option<String>,
}

impl From<HealthEventDetailsRow> for HealthEventDetails {
    fn from(row: HealthEventDetailsRow) -> Self {
        Self {
            id: row.id,
            horse_id: row.horse_id,
            horse_name: row.horse_name,
            event_type: row.event_type,
            occurred_on: row.occurred_on,
            occurred_time: row.occurred_time,
            title: row.title,
            notes: row.notes,
            cost: row.cost,
            status: row.status,
            created_at: row.created_at,
            updated_at: row.updated_at,
            archived_at: row.archived_at,
        }
    }
}

#[derive(Debug, FromRow)]
struct HealthEventProfileRow {
    id: String,
    horse_id: String,
    event_type: String,
    occurred_on: String,
    occurred_time: Option<String>,
    title: String,
    notes: Option<String>,
    cost: Option<f64>,
    status: String,
    created_at: String,
    updated_at: String,
    archived_at: Option<String>,
}

impl TryFrom<HealthEventProfileRow> for HealthEventProfileData {
    type Error = String;

    fn try_from(row: HealthEventProfileRow) -> Result<Self, Self::Error> {
        let event = HealthEvent::from_existing(
            HealthEventId::from_string(row.id),
            HorseReference::new(row.horse_id).map_err(|error| error.to_string())?,
            HealthEventType::try_from(row.event_type.as_str())
                .map_err(|error| error.to_string())?,
            HealthEventDate::new(row.occurred_on).map_err(|error| error.to_string())?,
            HealthEventTitle::new(row.title).map_err(|error| error.to_string())?,
            HealthEventStatus::try_from(row.status.as_str())?,
        );

        Ok(Self {
            event,
            occurred_time: row.occurred_time,
            notes: row.notes,
            cost: row.cost,
            created_at: row.created_at,
            updated_at: row.updated_at,
            archived_at: row.archived_at,
        })
    }
}
