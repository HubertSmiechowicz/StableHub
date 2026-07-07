use serde::Deserialize;
use tauri::State;

use crate::{
    modules::inventory::{
        application::{
            commands::{
                apply_inventory_usage::{ApplyInventoryUsageCommand, ApplyInventoryUsageHandler},
                archive_inventory_item::{
                    ArchiveInventoryItemCommand, ArchiveInventoryItemHandler,
                },
                create_inventory_item::{CreateInventoryItemCommand, CreateInventoryItemHandler},
                record_inventory_stocktake::{
                    RecordInventoryStocktakeCommand, RecordInventoryStocktakeHandler,
                },
                register_inventory_delivery::{
                    RegisterInventoryDeliveryCommand, RegisterInventoryDeliveryHandler,
                },
                update_inventory_item::{UpdateInventoryItemCommand, UpdateInventoryItemHandler},
            },
            dto::{InventoryItemDetails, InventoryItemSummary},
            queries::{
                get_inventory_item_details::{
                    GetInventoryItemDetailsHandler, GetInventoryItemDetailsQuery,
                },
                list_inventory_items::{ListInventoryItemsHandler, ListInventoryItemsQuery},
            },
        },
        infrastructure::persistence::sqlite::repositories::SqliteInventoryRepository,
    },
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct CreateInventoryItemRequest {
    pub name: String,
    pub unit: String,
    pub minimum_quantity: Option<f64>,
    pub daily_usage: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateInventoryItemRequest {
    pub id: String,
    pub name: String,
    pub unit: String,
    pub minimum_quantity: Option<f64>,
    pub daily_usage: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct ListInventoryItemsRequest {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_direction: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterInventoryDeliveryRequest {
    pub inventory_item_id: String,
    pub delivered_on: String,
    pub quantity: f64,
    pub total_cost: f64,
    pub supplier: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RecordInventoryStocktakeRequest {
    pub inventory_item_id: String,
    pub counted_on: String,
    pub actual_quantity: f64,
    pub notes: Option<String>,
}

#[tauri::command]
pub async fn create_inventory_item(
    request: CreateInventoryItemRequest,
    state: State<'_, AppState>,
) -> Result<InventoryItemDetails, String> {
    let repository = SqliteInventoryRepository::new(state.db.clone());
    let handler = CreateInventoryItemHandler::new(&repository);

    handler
        .handle(CreateInventoryItemCommand {
            name: request.name,
            unit: request.unit,
            minimum_quantity: request.minimum_quantity,
            daily_usage: request.daily_usage,
        })
        .await
}

#[tauri::command]
pub async fn update_inventory_item(
    request: UpdateInventoryItemRequest,
    state: State<'_, AppState>,
) -> Result<InventoryItemDetails, String> {
    let repository = SqliteInventoryRepository::new(state.db.clone());
    let handler = UpdateInventoryItemHandler::new(&repository);

    handler
        .handle(UpdateInventoryItemCommand {
            id: request.id,
            name: request.name,
            unit: request.unit,
            minimum_quantity: request.minimum_quantity,
            daily_usage: request.daily_usage,
        })
        .await
}

#[tauri::command]
pub async fn archive_inventory_item(id: String, state: State<'_, AppState>) -> Result<(), String> {
    let repository = SqliteInventoryRepository::new(state.db.clone());
    let handler = ArchiveInventoryItemHandler::new(&repository);

    handler.handle(ArchiveInventoryItemCommand { id }).await
}

#[tauri::command]
pub async fn list_inventory_items(
    request: Option<ListInventoryItemsRequest>,
    state: State<'_, AppState>,
) -> Result<Vec<InventoryItemSummary>, String> {
    let repository = SqliteInventoryRepository::new(state.db.clone());
    let handler = ListInventoryItemsHandler::new(&repository);
    let request = request.unwrap_or(ListInventoryItemsRequest {
        search: None,
        sort_by: None,
        sort_direction: None,
    });

    handler
        .handle(ListInventoryItemsQuery {
            search: request.search,
            sort_by: request.sort_by.unwrap_or_else(|| "name".to_string()),
            sort_direction: request.sort_direction.unwrap_or_else(|| "asc".to_string()),
        })
        .await
}

#[tauri::command]
pub async fn get_inventory_item_details(
    id: String,
    state: State<'_, AppState>,
) -> Result<InventoryItemDetails, String> {
    let repository = SqliteInventoryRepository::new(state.db.clone());
    let handler = GetInventoryItemDetailsHandler::new(&repository);

    handler.handle(GetInventoryItemDetailsQuery { id }).await
}

#[tauri::command]
pub async fn register_inventory_delivery(
    request: RegisterInventoryDeliveryRequest,
    state: State<'_, AppState>,
) -> Result<InventoryItemDetails, String> {
    let repository = SqliteInventoryRepository::new(state.db.clone());
    let handler = RegisterInventoryDeliveryHandler::new(&repository);

    handler
        .handle(RegisterInventoryDeliveryCommand {
            inventory_item_id: request.inventory_item_id,
            delivered_on: request.delivered_on,
            quantity: request.quantity,
            total_cost: request.total_cost,
            supplier: request.supplier,
            notes: request.notes,
        })
        .await
}

#[tauri::command]
pub async fn apply_inventory_usage(
    id: String,
    state: State<'_, AppState>,
) -> Result<InventoryItemDetails, String> {
    let repository = SqliteInventoryRepository::new(state.db.clone());
    let handler = ApplyInventoryUsageHandler::new(&repository);

    handler
        .handle(ApplyInventoryUsageCommand {
            inventory_item_id: id,
        })
        .await
}

#[tauri::command]
pub async fn record_inventory_stocktake(
    request: RecordInventoryStocktakeRequest,
    state: State<'_, AppState>,
) -> Result<InventoryItemDetails, String> {
    let repository = SqliteInventoryRepository::new(state.db.clone());
    let handler = RecordInventoryStocktakeHandler::new(&repository);

    handler
        .handle(RecordInventoryStocktakeCommand {
            inventory_item_id: request.inventory_item_id,
            counted_on: request.counted_on,
            actual_quantity: request.actual_quantity,
            notes: request.notes,
        })
        .await
}
