use serde::Deserialize;
use tauri::State;

use crate::{
    modules::health::{
        application::{
            commands::{
                archive_health_event::{ArchiveHealthEventCommand, ArchiveHealthEventHandler},
                create_health_event::{CreateHealthEventCommand, CreateHealthEventHandler},
                update_health_event::{UpdateHealthEventCommand, UpdateHealthEventHandler},
            },
            dto::{HealthEventDetails, HealthEventSummary},
            queries::{
                get_health_event_details::{
                    GetHealthEventDetailsHandler, GetHealthEventDetailsQuery,
                },
                list_health_events::{ListHealthEventsHandler, ListHealthEventsQuery},
            },
        },
        infrastructure::persistence::sqlite::repositories::SqliteHealthEventRepository,
    },
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct CreateHealthEventRequest {
    pub horse_id: String,
    pub event_type: String,
    pub occurred_on: String,
    pub title: String,
    pub notes: Option<String>,
    pub cost: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateHealthEventRequest {
    pub id: String,
    pub horse_id: String,
    pub event_type: String,
    pub occurred_on: String,
    pub title: String,
    pub notes: Option<String>,
    pub cost: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct ListHealthEventsRequest {
    pub search: Option<String>,
    pub horse_id: Option<String>,
    pub event_type: Option<String>,
    pub sort_by: Option<String>,
    pub sort_direction: Option<String>,
}

#[tauri::command]
pub async fn create_health_event(
    request: CreateHealthEventRequest,
    state: State<'_, AppState>,
) -> Result<HealthEventDetails, String> {
    let repository = SqliteHealthEventRepository::new(state.db.clone());
    let handler = CreateHealthEventHandler::new(&repository);

    handler
        .handle(CreateHealthEventCommand {
            horse_id: request.horse_id,
            event_type: request.event_type,
            occurred_on: request.occurred_on,
            title: request.title,
            notes: request.notes,
            cost: request.cost,
        })
        .await
}

#[tauri::command]
pub async fn update_health_event(
    request: UpdateHealthEventRequest,
    state: State<'_, AppState>,
) -> Result<HealthEventDetails, String> {
    let repository = SqliteHealthEventRepository::new(state.db.clone());
    let handler = UpdateHealthEventHandler::new(&repository);

    handler
        .handle(UpdateHealthEventCommand {
            id: request.id,
            horse_id: request.horse_id,
            event_type: request.event_type,
            occurred_on: request.occurred_on,
            title: request.title,
            notes: request.notes,
            cost: request.cost,
        })
        .await
}

#[tauri::command]
pub async fn archive_health_event(id: String, state: State<'_, AppState>) -> Result<(), String> {
    let repository = SqliteHealthEventRepository::new(state.db.clone());
    let handler = ArchiveHealthEventHandler::new(&repository);

    handler.handle(ArchiveHealthEventCommand { id }).await
}

#[tauri::command]
pub async fn list_health_events(
    request: Option<ListHealthEventsRequest>,
    state: State<'_, AppState>,
) -> Result<Vec<HealthEventSummary>, String> {
    let repository = SqliteHealthEventRepository::new(state.db.clone());
    let handler = ListHealthEventsHandler::new(&repository);
    let request = request.unwrap_or(ListHealthEventsRequest {
        search: None,
        horse_id: None,
        event_type: None,
        sort_by: None,
        sort_direction: None,
    });

    handler
        .handle(ListHealthEventsQuery {
            search: request.search,
            horse_id: request.horse_id,
            event_type: request.event_type,
            sort_by: request.sort_by.unwrap_or_else(|| "occurred_on".to_string()),
            sort_direction: request.sort_direction.unwrap_or_else(|| "desc".to_string()),
        })
        .await
}

#[tauri::command]
pub async fn get_health_event_details(
    id: String,
    state: State<'_, AppState>,
) -> Result<HealthEventDetails, String> {
    let repository = SqliteHealthEventRepository::new(state.db.clone());
    let handler = GetHealthEventDetailsHandler::new(&repository);

    handler.handle(GetHealthEventDetailsQuery { id }).await
}
