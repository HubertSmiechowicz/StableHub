use serde::Deserialize;
use tauri::State;

use crate::{
    modules::calendar::{
        application::{
            commands::{
                archive_calendar_entry::{
                    ArchiveCalendarEntryCommand, ArchiveCalendarEntryHandler,
                },
                create_calendar_entry::{CreateCalendarEntryCommand, CreateCalendarEntryHandler},
                update_calendar_entry::{UpdateCalendarEntryCommand, UpdateCalendarEntryHandler},
            },
            dto::{CalendarEntryDetails, CalendarItemSummary},
            queries::{
                get_calendar_entry_details::{
                    GetCalendarEntryDetailsHandler, GetCalendarEntryDetailsQuery,
                },
                list_calendar_items::{ListCalendarItemsHandler, ListCalendarItemsQuery},
            },
        },
        infrastructure::persistence::sqlite::repositories::SqliteCalendarRepository,
    },
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct CreateCalendarEntryRequest {
    pub title: String,
    pub scheduled_on: String,
    pub scheduled_time: Option<String>,
    pub entry_type: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCalendarEntryRequest {
    pub id: String,
    pub title: String,
    pub scheduled_on: String,
    pub scheduled_time: Option<String>,
    pub entry_type: String,
    pub description: Option<String>,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct ListCalendarItemsRequest {
    pub search: Option<String>,
    pub source_module: Option<String>,
    pub item_type: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub sort_by: Option<String>,
    pub sort_direction: Option<String>,
}

#[tauri::command]
pub async fn create_calendar_entry(
    request: CreateCalendarEntryRequest,
    state: State<'_, AppState>,
) -> Result<CalendarEntryDetails, String> {
    let repository = SqliteCalendarRepository::new(state.db.clone());
    let handler = CreateCalendarEntryHandler::new(&repository);

    handler
        .handle(CreateCalendarEntryCommand {
            title: request.title,
            scheduled_on: request.scheduled_on,
            scheduled_time: request.scheduled_time,
            entry_type: request.entry_type,
            description: request.description,
        })
        .await
}

#[tauri::command]
pub async fn update_calendar_entry(
    request: UpdateCalendarEntryRequest,
    state: State<'_, AppState>,
) -> Result<CalendarEntryDetails, String> {
    let repository = SqliteCalendarRepository::new(state.db.clone());
    let handler = UpdateCalendarEntryHandler::new(&repository);

    handler
        .handle(UpdateCalendarEntryCommand {
            id: request.id,
            title: request.title,
            scheduled_on: request.scheduled_on,
            scheduled_time: request.scheduled_time,
            entry_type: request.entry_type,
            description: request.description,
            status: request.status,
        })
        .await
}

#[tauri::command]
pub async fn archive_calendar_entry(id: String, state: State<'_, AppState>) -> Result<(), String> {
    let repository = SqliteCalendarRepository::new(state.db.clone());
    let handler = ArchiveCalendarEntryHandler::new(&repository);

    handler.handle(ArchiveCalendarEntryCommand { id }).await
}

#[tauri::command]
pub async fn list_calendar_items(
    request: Option<ListCalendarItemsRequest>,
    state: State<'_, AppState>,
) -> Result<Vec<CalendarItemSummary>, String> {
    let repository = SqliteCalendarRepository::new(state.db.clone());
    let handler = ListCalendarItemsHandler::new(&repository);
    let request = request.unwrap_or(ListCalendarItemsRequest {
        search: None,
        source_module: None,
        item_type: None,
        date_from: None,
        date_to: None,
        sort_by: None,
        sort_direction: None,
    });

    handler
        .handle(ListCalendarItemsQuery {
            search: request.search,
            source_module: request.source_module,
            item_type: request.item_type,
            date_from: request.date_from,
            date_to: request.date_to,
            sort_by: request
                .sort_by
                .unwrap_or_else(|| "scheduled_on".to_string()),
            sort_direction: request.sort_direction.unwrap_or_else(|| "asc".to_string()),
        })
        .await
}

#[tauri::command]
pub async fn get_calendar_entry_details(
    id: String,
    state: State<'_, AppState>,
) -> Result<CalendarEntryDetails, String> {
    let repository = SqliteCalendarRepository::new(state.db.clone());
    let handler = GetCalendarEntryDetailsHandler::new(&repository);

    handler.handle(GetCalendarEntryDetailsQuery { id }).await
}
