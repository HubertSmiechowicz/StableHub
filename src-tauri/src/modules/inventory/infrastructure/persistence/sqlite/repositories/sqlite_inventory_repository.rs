use async_trait::async_trait;
use sqlx::{FromRow, SqlitePool};

use crate::modules::inventory::application::{
    dto::{InventoryItemDetails, InventoryItemProfileData, InventoryItemSummary},
    ports::{InventoryListOptions, InventoryRepository, InventorySortBy, SortDirection},
};
use crate::modules::inventory::domain::{
    InventoryItem, InventoryItemId, InventoryItemName, InventoryItemStatus, InventoryUnit,
    StockLevel,
};

pub struct SqliteInventoryRepository {
    pool: SqlitePool,
}

impl SqliteInventoryRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl InventoryRepository for SqliteInventoryRepository {
    async fn save_item(&self, profile: &InventoryItemProfileData) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO inventory_items (
                id,
                name,
                unit,
                quantity,
                minimum_quantity,
                daily_usage,
                status,
                created_at,
                updated_at,
                archived_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                unit = excluded.unit,
                quantity = excluded.quantity,
                minimum_quantity = excluded.minimum_quantity,
                daily_usage = excluded.daily_usage,
                status = excluded.status,
                updated_at = excluded.updated_at,
                archived_at = excluded.archived_at
            "#,
        )
        .bind(profile.item.id().as_str())
        .bind(profile.item.name().as_str())
        .bind(profile.item.unit().as_str())
        .bind(profile.item.stock_level().value())
        .bind(profile.minimum_quantity)
        .bind(profile.daily_usage)
        .bind(profile.item.status().as_str())
        .bind(profile.created_at.as_str())
        .bind(profile.updated_at.as_str())
        .bind(profile.archived_at.as_deref())
        .execute(&self.pool)
        .await
        .map_err(|error| format!("Nie udało się zapisać pozycji magazynowej: {error}"))?;

        Ok(())
    }

    async fn find_profile_by_id(
        &self,
        id: &str,
    ) -> Result<Option<InventoryItemProfileData>, String> {
        let row = sqlx::query_as::<_, InventoryItemProfileRow>(
            r#"
            SELECT
                id,
                name,
                unit,
                quantity,
                minimum_quantity,
                daily_usage,
                status,
                created_at,
                updated_at,
                archived_at
            FROM inventory_items
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| format!("Nie udało się pobrać profilu pozycji magazynowej: {error}"))?;

        row.map(InventoryItemProfileData::try_from).transpose()
    }

    async fn active_item_exists(
        &self,
        name: &str,
        unit: &str,
        except_id: Option<&str>,
    ) -> Result<bool, String> {
        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM inventory_items
            WHERE status = 'active'
              AND unit = ?
              AND lower(name) = lower(?)
              AND (? IS NULL OR id != ?)
            "#,
        )
        .bind(unit)
        .bind(name.trim())
        .bind(except_id)
        .bind(except_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|error| {
            format!("Nie udało się sprawdzić unikalności pozycji magazynowej: {error}")
        })?;

        Ok(count > 0)
    }

    async fn list_active_summaries(
        &self,
        options: &InventoryListOptions,
    ) -> Result<Vec<InventoryItemSummary>, String> {
        let order_by = match options.sort_by {
            InventorySortBy::Name => "name COLLATE NOCASE",
            InventorySortBy::Quantity => "quantity",
            InventorySortBy::DaysRemaining => {
                "CASE WHEN daily_usage IS NULL OR daily_usage <= 0 THEN NULL ELSE quantity / daily_usage END"
            }
            InventorySortBy::CreatedAt => "created_at",
        };
        let direction = match options.sort_direction {
            SortDirection::Asc => "ASC",
            SortDirection::Desc => "DESC",
        };
        let sql = format!(
            r#"
            SELECT
                id,
                name,
                unit,
                quantity,
                minimum_quantity,
                daily_usage,
                status
            FROM inventory_items
            WHERE status = 'active'
              AND (? IS NULL OR lower(name) LIKE lower(?))
            ORDER BY {order_by} {direction}, name COLLATE NOCASE ASC
            "#
        );
        let search = options.search.as_ref().map(|value| format!("%{value}%"));

        let rows = sqlx::query_as::<_, InventoryItemSummaryRow>(&sql)
            .bind(search.as_deref())
            .bind(search.as_deref())
            .fetch_all(&self.pool)
            .await
            .map_err(|error| format!("Nie udało się pobrać listy pozycji magazynowych: {error}"))?;

        Ok(rows.into_iter().map(InventoryItemSummary::from).collect())
    }

    async fn find_details_by_id(&self, id: &str) -> Result<Option<InventoryItemDetails>, String> {
        let row = sqlx::query_as::<_, InventoryItemDetailsRow>(
            r#"
            SELECT
                id,
                name,
                unit,
                quantity,
                minimum_quantity,
                daily_usage,
                status,
                created_at,
                updated_at,
                archived_at
            FROM inventory_items
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| format!("Nie udało się pobrać pozycji magazynowej: {error}"))?;

        Ok(row.map(InventoryItemDetails::from))
    }
}

#[derive(Debug, FromRow)]
struct InventoryItemProfileRow {
    id: String,
    name: String,
    unit: String,
    quantity: f64,
    minimum_quantity: Option<f64>,
    daily_usage: Option<f64>,
    status: String,
    created_at: String,
    updated_at: String,
    archived_at: Option<String>,
}

impl TryFrom<InventoryItemProfileRow> for InventoryItemProfileData {
    type Error = String;

    fn try_from(row: InventoryItemProfileRow) -> Result<Self, Self::Error> {
        let item = InventoryItem::from_existing(
            InventoryItemId::from_string(row.id),
            InventoryItemName::new(row.name).map_err(|error| error.to_string())?,
            InventoryUnit::try_from(row.unit.as_str()).map_err(|error| error.to_string())?,
            StockLevel::new(row.quantity).map_err(|error| error.to_string())?,
            InventoryItemStatus::try_from(row.status.as_str())?,
        );

        Ok(Self {
            item,
            minimum_quantity: row.minimum_quantity,
            daily_usage: row.daily_usage,
            created_at: row.created_at,
            updated_at: row.updated_at,
            archived_at: row.archived_at,
        })
    }
}

#[derive(Debug, FromRow)]
struct InventoryItemSummaryRow {
    id: String,
    name: String,
    unit: String,
    quantity: f64,
    minimum_quantity: Option<f64>,
    daily_usage: Option<f64>,
    status: String,
}

impl From<InventoryItemSummaryRow> for InventoryItemSummary {
    fn from(row: InventoryItemSummaryRow) -> Self {
        let days_remaining = calculate_days_remaining(row.quantity, row.daily_usage);

        Self {
            id: row.id,
            name: row.name,
            unit: row.unit,
            quantity: row.quantity,
            minimum_quantity: row.minimum_quantity,
            daily_usage: row.daily_usage,
            days_remaining,
            status: row.status,
        }
    }
}

#[derive(Debug, FromRow)]
struct InventoryItemDetailsRow {
    id: String,
    name: String,
    unit: String,
    quantity: f64,
    minimum_quantity: Option<f64>,
    daily_usage: Option<f64>,
    status: String,
    created_at: String,
    updated_at: String,
    archived_at: Option<String>,
}

impl From<InventoryItemDetailsRow> for InventoryItemDetails {
    fn from(row: InventoryItemDetailsRow) -> Self {
        let days_remaining = calculate_days_remaining(row.quantity, row.daily_usage);

        Self {
            id: row.id,
            name: row.name,
            unit: row.unit,
            quantity: row.quantity,
            minimum_quantity: row.minimum_quantity,
            daily_usage: row.daily_usage,
            days_remaining,
            status: row.status,
            created_at: row.created_at,
            updated_at: row.updated_at,
            archived_at: row.archived_at,
        }
    }
}

fn calculate_days_remaining(quantity: f64, daily_usage: Option<f64>) -> Option<f64> {
    daily_usage
        .filter(|usage| *usage > 0.0)
        .map(|usage| quantity / usage)
}
