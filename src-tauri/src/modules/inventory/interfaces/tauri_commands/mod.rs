use serde::Deserialize;
use tauri::State;

use crate::{
    modules::inventory::{
        application::{
            commands::create_inventory_item::{
                CreateInventoryItemCommand, CreateInventoryItemHandler,
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
pub async fn list_inventory_items(
    state: State<'_, AppState>,
) -> Result<Vec<InventoryItemSummary>, String> {
    let repository = SqliteInventoryRepository::new(state.db.clone());
    let handler = ListInventoryItemsHandler::new(&repository);

    handler.handle(ListInventoryItemsQuery).await
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
