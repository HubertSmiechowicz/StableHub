use serde::Deserialize;
use tauri::State;

use crate::{
    modules::inventory::{
        application::{
            commands::{
                archive_inventory_item::{
                    ArchiveInventoryItemCommand, ArchiveInventoryItemHandler,
                },
                create_inventory_item::{CreateInventoryItemCommand, CreateInventoryItemHandler},
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
    pub quantity: f64,
    pub minimum_quantity: Option<f64>,
    pub daily_usage: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateInventoryItemRequest {
    pub id: String,
    pub name: String,
    pub unit: String,
    pub quantity: f64,
    pub minimum_quantity: Option<f64>,
    pub daily_usage: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct ListInventoryItemsRequest {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_direction: Option<String>,
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
            quantity: request.quantity,
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
            quantity: request.quantity,
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
