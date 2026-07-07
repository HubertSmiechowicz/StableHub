use async_trait::async_trait;
use sqlx::{FromRow, SqlitePool};

use crate::modules::inventory::application::{
    dto::{
        InventoryDeliveryData, InventoryDeliverySummary, InventoryItemDetails,
        InventoryItemProfileData, InventoryItemSummary, InventoryStocktakeData,
        InventoryStocktakeSummary,
    },
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
                archived_at,
                last_usage_applied_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                unit = excluded.unit,
                quantity = excluded.quantity,
                minimum_quantity = excluded.minimum_quantity,
                daily_usage = excluded.daily_usage,
                status = excluded.status,
                updated_at = excluded.updated_at,
                archived_at = excluded.archived_at,
                last_usage_applied_at = excluded.last_usage_applied_at
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
        .bind(profile.last_usage_applied_at.as_deref())
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
                archived_at,
                last_usage_applied_at
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
                archived_at,
                last_usage_applied_at
            FROM inventory_items
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| format!("Nie udało się pobrać pozycji magazynowej: {error}"))?;

        let Some(row) = row else {
            return Ok(None);
        };

        let recent_deliveries = sqlx::query_as::<_, InventoryDeliveryRow>(
            r#"
            SELECT
                id,
                inventory_item_id,
                delivered_on,
                quantity,
                total_cost,
                supplier,
                notes,
                created_at
            FROM inventory_deliveries
            WHERE inventory_item_id = ?
            ORDER BY delivered_on DESC, created_at DESC
            LIMIT 5
            "#,
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        .map_err(|error| format!("Nie udało się pobrać dostaw pozycji magazynowej: {error}"))?
        .into_iter()
        .map(InventoryDeliverySummary::from)
        .collect::<Vec<_>>();

        let aggregate = sqlx::query_as::<_, InventoryDeliveryAggregateRow>(
            r#"
            SELECT
                COALESCE(SUM(total_cost), 0.0) AS total_delivery_cost,
                COALESCE(SUM(quantity), 0.0) AS total_delivery_quantity
            FROM inventory_deliveries
            WHERE inventory_item_id = ?
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|error| format!("Nie udało się pobrać kosztów pozycji magazynowej: {error}"))?;

        let recent_stocktakes = sqlx::query_as::<_, InventoryStocktakeRow>(
            r#"
            SELECT
                id,
                inventory_item_id,
                counted_on,
                expected_quantity,
                actual_quantity,
                variance_quantity,
                expected_usage,
                notes,
                created_at
            FROM inventory_stocktakes
            WHERE inventory_item_id = ?
            ORDER BY counted_on DESC, created_at DESC
            LIMIT 5
            "#,
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        .map_err(|error| {
            format!("Nie udało się pobrać inwentaryzacji pozycji magazynowej: {error}")
        })?
        .into_iter()
        .map(InventoryStocktakeSummary::from)
        .collect::<Vec<_>>();

        Ok(Some(InventoryItemDetails::from_parts(
            row,
            recent_deliveries,
            recent_stocktakes,
            aggregate.total_delivery_cost,
            aggregate.total_delivery_quantity,
        )))
    }

    async fn register_delivery(&self, delivery: &InventoryDeliveryData) -> Result<(), String> {
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|error| format!("Nie udało się rozpocząć zapisu dostawy: {error}"))?;

        sqlx::query(
            r#"
            INSERT INTO inventory_deliveries (
                id,
                inventory_item_id,
                delivered_on,
                quantity,
                total_cost,
                supplier,
                notes,
                created_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(delivery.id.as_str())
        .bind(delivery.inventory_item_id.as_str())
        .bind(delivery.delivered_on.as_str())
        .bind(delivery.quantity)
        .bind(delivery.total_cost)
        .bind(delivery.supplier.as_deref())
        .bind(delivery.notes.as_deref())
        .bind(delivery.created_at.as_str())
        .execute(&mut *transaction)
        .await
        .map_err(|error| format!("Nie udało się zapisać dostawy magazynowej: {error}"))?;

        let result = sqlx::query(
            r#"
            UPDATE inventory_items
            SET quantity = quantity + ?,
                updated_at = ?
            WHERE id = ?
              AND status = 'active'
            "#,
        )
        .bind(delivery.quantity)
        .bind(delivery.created_at.as_str())
        .bind(delivery.inventory_item_id.as_str())
        .execute(&mut *transaction)
        .await
        .map_err(|error| format!("Nie udało się zwiększyć stanu magazynowego: {error}"))?;

        if result.rows_affected() == 0 {
            return Err("Nie znaleziono aktywnej pozycji magazynowej dla dostawy".to_string());
        }

        transaction
            .commit()
            .await
            .map_err(|error| format!("Nie udało się zatwierdzić dostawy magazynowej: {error}"))?;

        Ok(())
    }

    async fn apply_usage(
        &self,
        item_id: &str,
        new_quantity: f64,
        applied_at: &str,
    ) -> Result<(), String> {
        let result = sqlx::query(
            r#"
            UPDATE inventory_items
            SET quantity = ?,
                last_usage_applied_at = ?,
                updated_at = ?
            WHERE id = ?
              AND status = 'active'
            "#,
        )
        .bind(new_quantity)
        .bind(applied_at)
        .bind(applied_at)
        .bind(item_id)
        .execute(&self.pool)
        .await
        .map_err(|error| format!("Nie udało się rozliczyć zużycia magazynowego: {error}"))?;

        if result.rows_affected() == 0 {
            return Err("Nie znaleziono aktywnej pozycji magazynowej do rozliczenia".to_string());
        }

        Ok(())
    }

    async fn record_stocktake(&self, stocktake: &InventoryStocktakeData) -> Result<(), String> {
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|error| format!("Nie udało się rozpocząć inwentaryzacji: {error}"))?;

        sqlx::query(
            r#"
            INSERT INTO inventory_stocktakes (
                id,
                inventory_item_id,
                counted_on,
                expected_quantity,
                actual_quantity,
                variance_quantity,
                expected_usage,
                notes,
                created_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(stocktake.id.as_str())
        .bind(stocktake.inventory_item_id.as_str())
        .bind(stocktake.counted_on.as_str())
        .bind(stocktake.expected_quantity)
        .bind(stocktake.actual_quantity)
        .bind(stocktake.variance_quantity)
        .bind(stocktake.expected_usage)
        .bind(stocktake.notes.as_deref())
        .bind(stocktake.created_at.as_str())
        .execute(&mut *transaction)
        .await
        .map_err(|error| format!("Nie udało się zapisać inwentaryzacji: {error}"))?;

        let result = sqlx::query(
            r#"
            UPDATE inventory_items
            SET quantity = ?,
                last_usage_applied_at = ?,
                updated_at = ?
            WHERE id = ?
              AND status = 'active'
            "#,
        )
        .bind(stocktake.actual_quantity)
        .bind(stocktake.created_at.as_str())
        .bind(stocktake.created_at.as_str())
        .bind(stocktake.inventory_item_id.as_str())
        .execute(&mut *transaction)
        .await
        .map_err(|error| format!("Nie udało się ustawić stanu po inwentaryzacji: {error}"))?;

        if result.rows_affected() == 0 {
            return Err(
                "Nie znaleziono aktywnej pozycji magazynowej do inwentaryzacji".to_string(),
            );
        }

        transaction
            .commit()
            .await
            .map_err(|error| format!("Nie udało się zatwierdzić inwentaryzacji: {error}"))?;

        Ok(())
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
    last_usage_applied_at: Option<String>,
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
            last_usage_applied_at: row.last_usage_applied_at,
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
    last_usage_applied_at: Option<String>,
}

impl InventoryItemDetails {
    fn from_parts(
        row: InventoryItemDetailsRow,
        recent_deliveries: Vec<InventoryDeliverySummary>,
        recent_stocktakes: Vec<InventoryStocktakeSummary>,
        total_delivery_cost: f64,
        total_delivery_quantity: f64,
    ) -> Self {
        let days_remaining = calculate_days_remaining(row.quantity, row.daily_usage);
        let average_unit_cost = (total_delivery_quantity > 0.0)
            .then_some(total_delivery_cost / total_delivery_quantity);
        let pending_usage_days = calculate_pending_usage_days(row.last_usage_applied_at.as_deref());
        let pending_usage_quantity = row
            .daily_usage
            .filter(|usage| *usage > 0.0)
            .map(|usage| usage * pending_usage_days as f64)
            .unwrap_or(0.0);

        Self {
            id: row.id,
            name: row.name,
            unit: row.unit,
            quantity: row.quantity,
            minimum_quantity: row.minimum_quantity,
            daily_usage: row.daily_usage,
            days_remaining,
            status: row.status,
            recent_deliveries,
            recent_stocktakes,
            total_delivery_cost,
            average_unit_cost,
            last_usage_applied_at: row.last_usage_applied_at,
            pending_usage_days,
            pending_usage_quantity,
            created_at: row.created_at,
            updated_at: row.updated_at,
            archived_at: row.archived_at,
        }
    }
}

#[derive(Debug, FromRow)]
struct InventoryDeliveryRow {
    id: String,
    inventory_item_id: String,
    delivered_on: String,
    quantity: f64,
    total_cost: f64,
    supplier: Option<String>,
    notes: Option<String>,
    created_at: String,
}

impl From<InventoryDeliveryRow> for InventoryDeliverySummary {
    fn from(row: InventoryDeliveryRow) -> Self {
        let unit_cost = (row.quantity > 0.0).then_some(row.total_cost / row.quantity);

        Self {
            id: row.id,
            inventory_item_id: row.inventory_item_id,
            delivered_on: row.delivered_on,
            quantity: row.quantity,
            total_cost: row.total_cost,
            unit_cost,
            supplier: row.supplier,
            notes: row.notes,
            created_at: row.created_at,
        }
    }
}

#[derive(Debug, FromRow)]
struct InventoryDeliveryAggregateRow {
    total_delivery_cost: f64,
    total_delivery_quantity: f64,
}

#[derive(Debug, FromRow)]
struct InventoryStocktakeRow {
    id: String,
    inventory_item_id: String,
    counted_on: String,
    expected_quantity: f64,
    actual_quantity: f64,
    variance_quantity: f64,
    expected_usage: f64,
    notes: Option<String>,
    created_at: String,
}

impl From<InventoryStocktakeRow> for InventoryStocktakeSummary {
    fn from(row: InventoryStocktakeRow) -> Self {
        Self {
            id: row.id,
            inventory_item_id: row.inventory_item_id,
            counted_on: row.counted_on,
            expected_quantity: row.expected_quantity,
            actual_quantity: row.actual_quantity,
            variance_quantity: row.variance_quantity,
            expected_usage: row.expected_usage,
            notes: row.notes,
            created_at: row.created_at,
        }
    }
}

fn calculate_days_remaining(quantity: f64, daily_usage: Option<f64>) -> Option<f64> {
    daily_usage
        .filter(|usage| *usage > 0.0)
        .map(|usage| quantity / usage)
}

fn calculate_pending_usage_days(last_usage_applied_at: Option<&str>) -> i64 {
    let Some(last_usage_applied_at) = last_usage_applied_at else {
        return 0;
    };

    let Ok(last) = last_usage_applied_at.parse::<u64>() else {
        return 0;
    };

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0);

    if now <= last {
        return 0;
    }

    ((now - last) / 86_400) as i64
}
