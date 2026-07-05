use async_trait::async_trait;
use sqlx::{FromRow, SqlitePool};

use crate::modules::calendar::{
    application::{
        dto::{CalendarEntryDetails, CalendarEntryProfileData, CalendarItemSummary},
        ports::{CalendarItemSortBy, CalendarListOptions, CalendarRepository, SortDirection},
    },
    domain::{
        CalendarEntry, CalendarEntryDate, CalendarEntryId, CalendarEntryStatus, CalendarEntryTitle,
        CalendarEntryType,
    },
};

pub struct SqliteCalendarRepository {
    pool: SqlitePool,
}

impl SqliteCalendarRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CalendarRepository for SqliteCalendarRepository {
    async fn save_entry(&self, profile: &CalendarEntryProfileData) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO calendar_entries (
                id,
                title,
                scheduled_on,
                scheduled_time,
                entry_type,
                description,
                status,
                created_at,
                updated_at,
                archived_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                title = excluded.title,
                scheduled_on = excluded.scheduled_on,
                scheduled_time = excluded.scheduled_time,
                entry_type = excluded.entry_type,
                description = excluded.description,
                status = excluded.status,
                updated_at = excluded.updated_at,
                archived_at = excluded.archived_at
            "#,
        )
        .bind(profile.entry.id().as_str())
        .bind(profile.entry.title().as_str())
        .bind(profile.entry.scheduled_on().as_str())
        .bind(profile.scheduled_time.as_deref())
        .bind(profile.entry.entry_type().as_str())
        .bind(profile.description.as_deref())
        .bind(profile.entry.status().as_str())
        .bind(profile.created_at.as_str())
        .bind(profile.updated_at.as_str())
        .bind(profile.archived_at.as_deref())
        .execute(&self.pool)
        .await
        .map_err(|error| format!("Nie udało się zapisać wpisu kalendarza: {error}"))?;

        Ok(())
    }

    async fn find_profile_by_id(
        &self,
        id: &str,
    ) -> Result<Option<CalendarEntryProfileData>, String> {
        let row = sqlx::query_as::<_, CalendarEntryProfileRow>(
            r#"
            SELECT
                id,
                title,
                scheduled_on,
                scheduled_time,
                entry_type,
                description,
                status,
                created_at,
                updated_at,
                archived_at
            FROM calendar_entries
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| format!("Nie udało się pobrać profilu wpisu kalendarza: {error}"))?;

        row.map(CalendarEntryProfileData::try_from).transpose()
    }

    async fn find_details_by_id(&self, id: &str) -> Result<Option<CalendarEntryDetails>, String> {
        let row = sqlx::query_as::<_, CalendarEntryDetailsRow>(
            r#"
            SELECT
                id,
                title,
                scheduled_on,
                scheduled_time,
                entry_type,
                description,
                status,
                created_at,
                updated_at,
                archived_at
            FROM calendar_entries
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| format!("Nie udało się pobrać wpisu kalendarza: {error}"))?;

        Ok(row.map(CalendarEntryDetails::from))
    }

    async fn list_items(
        &self,
        options: &CalendarListOptions,
    ) -> Result<Vec<CalendarItemSummary>, String> {
        let order_by = match options.sort_by {
            CalendarItemSortBy::ScheduledOn => "scheduled_on",
            CalendarItemSortBy::Title => "title COLLATE NOCASE",
            CalendarItemSortBy::Type => "item_type",
            CalendarItemSortBy::Source => "source_module",
        };
        let direction = match options.sort_direction {
            SortDirection::Asc => "ASC",
            SortDirection::Desc => "DESC",
        };
        let sql = format!(
            r#"
            SELECT *
            FROM (
                SELECT
                    id,
                    'calendar' AS source_module,
                    id AS source_id,
                    title,
                    scheduled_on,
                    scheduled_time,
                    entry_type AS item_type,
                    description,
                    status,
                    NULL AS related_label
                FROM calendar_entries
                WHERE status != 'archived'

                UNION ALL

                SELECT
                    health_events.id AS id,
                    'health' AS source_module,
                    health_events.id AS source_id,
                    health_events.title,
                    health_events.occurred_on AS scheduled_on,
                    health_events.occurred_time AS scheduled_time,
                    health_events.event_type AS item_type,
                    health_events.notes AS description,
                    health_events.status,
                    horses.name AS related_label
                FROM health_events
                LEFT JOIN horses ON horses.id = health_events.horse_id
                WHERE health_events.status = 'active'
            ) items
            WHERE (? IS NULL OR lower(title) LIKE lower(?) OR lower(COALESCE(description, '')) LIKE lower(?) OR lower(COALESCE(related_label, '')) LIKE lower(?))
              AND (? IS NULL OR source_module = ?)
              AND (? IS NULL OR item_type = ?)
              AND (? IS NULL OR scheduled_on >= ?)
              AND (? IS NULL OR scheduled_on <= ?)
            ORDER BY {order_by} {direction}, scheduled_time ASC, title COLLATE NOCASE ASC
            "#
        );
        let search = options.search.as_ref().map(|value| format!("%{value}%"));

        let rows = sqlx::query_as::<_, CalendarItemSummaryRow>(&sql)
            .bind(search.as_deref())
            .bind(search.as_deref())
            .bind(search.as_deref())
            .bind(search.as_deref())
            .bind(options.source_module.as_deref())
            .bind(options.source_module.as_deref())
            .bind(options.item_type.as_deref())
            .bind(options.item_type.as_deref())
            .bind(options.date_from.as_deref())
            .bind(options.date_from.as_deref())
            .bind(options.date_to.as_deref())
            .bind(options.date_to.as_deref())
            .fetch_all(&self.pool)
            .await
            .map_err(|error| format!("Nie udało się pobrać terminarza: {error}"))?;

        Ok(rows.into_iter().map(CalendarItemSummary::from).collect())
    }
}

#[derive(Debug, FromRow)]
struct CalendarItemSummaryRow {
    id: String,
    source_module: String,
    source_id: String,
    title: String,
    scheduled_on: String,
    scheduled_time: Option<String>,
    item_type: String,
    description: Option<String>,
    status: String,
    related_label: Option<String>,
}

impl From<CalendarItemSummaryRow> for CalendarItemSummary {
    fn from(row: CalendarItemSummaryRow) -> Self {
        Self {
            id: row.id,
            source_module: row.source_module,
            source_id: row.source_id,
            title: row.title,
            scheduled_on: row.scheduled_on,
            scheduled_time: row.scheduled_time,
            item_type: row.item_type,
            description: row.description,
            status: row.status,
            related_label: row.related_label,
        }
    }
}

#[derive(Debug, FromRow)]
struct CalendarEntryDetailsRow {
    id: String,
    title: String,
    scheduled_on: String,
    scheduled_time: Option<String>,
    entry_type: String,
    description: Option<String>,
    status: String,
    created_at: String,
    updated_at: String,
    archived_at: Option<String>,
}

impl From<CalendarEntryDetailsRow> for CalendarEntryDetails {
    fn from(row: CalendarEntryDetailsRow) -> Self {
        Self {
            id: row.id,
            title: row.title,
            scheduled_on: row.scheduled_on,
            scheduled_time: row.scheduled_time,
            entry_type: row.entry_type,
            description: row.description,
            status: row.status,
            created_at: row.created_at,
            updated_at: row.updated_at,
            archived_at: row.archived_at,
        }
    }
}

#[derive(Debug, FromRow)]
struct CalendarEntryProfileRow {
    id: String,
    title: String,
    scheduled_on: String,
    scheduled_time: Option<String>,
    entry_type: String,
    description: Option<String>,
    status: String,
    created_at: String,
    updated_at: String,
    archived_at: Option<String>,
}

impl TryFrom<CalendarEntryProfileRow> for CalendarEntryProfileData {
    type Error = String;

    fn try_from(row: CalendarEntryProfileRow) -> Result<Self, Self::Error> {
        let entry = CalendarEntry::from_existing(
            CalendarEntryId::from_string(row.id),
            CalendarEntryTitle::new(row.title).map_err(|error| error.to_string())?,
            CalendarEntryDate::new(row.scheduled_on).map_err(|error| error.to_string())?,
            CalendarEntryType::try_from(row.entry_type.as_str())
                .map_err(|error| error.to_string())?,
            CalendarEntryStatus::try_from(row.status.as_str())?,
        );

        Ok(Self {
            entry,
            scheduled_time: row.scheduled_time,
            description: row.description,
            created_at: row.created_at,
            updated_at: row.updated_at,
            archived_at: row.archived_at,
        })
    }
}
