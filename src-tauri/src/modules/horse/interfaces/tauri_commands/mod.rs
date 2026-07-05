use serde::Deserialize;
use tauri::State;

use crate::{
    modules::horse::{
        application::{
            commands::{
                archive_horse::{ArchiveHorseCommand, ArchiveHorseHandler},
                create_horse::{CreateHorseCommand, CreateHorseHandler},
                update_horse::{UpdateHorseCommand, UpdateHorseHandler},
            },
            dto::{HorseDetails, HorseSummary},
            queries::{
                get_horse_details::{GetHorseDetailsHandler, GetHorseDetailsQuery},
                list_horses::{ListHorsesHandler, ListHorsesQuery},
            },
        },
        infrastructure::persistence::sqlite::repositories::SqliteHorseRepository,
    },
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct CreateHorseRequest {
    pub name: String,
    pub sex: Option<String>,
    pub breed: Option<String>,
    pub date_of_birth: Option<String>,
    pub coat_color: Option<String>,
    pub identification_number: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateHorseRequest {
    pub id: String,
    pub name: String,
    pub sex: Option<String>,
    pub breed: Option<String>,
    pub date_of_birth: Option<String>,
    pub coat_color: Option<String>,
    pub identification_number: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListHorsesRequest {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_direction: Option<String>,
}

#[tauri::command]
pub async fn create_horse(
    request: CreateHorseRequest,
    state: State<'_, AppState>,
) -> Result<HorseDetails, String> {
    let repository = SqliteHorseRepository::new(state.db.clone());
    let handler = CreateHorseHandler::new(&repository);

    handler
        .handle(CreateHorseCommand {
            name: request.name,
            sex: request.sex,
            breed: request.breed,
            date_of_birth: request.date_of_birth,
            coat_color: request.coat_color,
            identification_number: request.identification_number,
            notes: request.notes,
        })
        .await
}

#[tauri::command]
pub async fn update_horse(
    request: UpdateHorseRequest,
    state: State<'_, AppState>,
) -> Result<HorseDetails, String> {
    let repository = SqliteHorseRepository::new(state.db.clone());
    let handler = UpdateHorseHandler::new(&repository);

    handler
        .handle(UpdateHorseCommand {
            id: request.id,
            name: request.name,
            sex: request.sex,
            breed: request.breed,
            date_of_birth: request.date_of_birth,
            coat_color: request.coat_color,
            identification_number: request.identification_number,
            notes: request.notes,
        })
        .await
}

#[tauri::command]
pub async fn archive_horse(id: String, state: State<'_, AppState>) -> Result<(), String> {
    let repository = SqliteHorseRepository::new(state.db.clone());
    let handler = ArchiveHorseHandler::new(&repository);

    handler.handle(ArchiveHorseCommand { id }).await
}

#[tauri::command]
pub async fn list_horses(
    request: Option<ListHorsesRequest>,
    state: State<'_, AppState>,
) -> Result<Vec<HorseSummary>, String> {
    let repository = SqliteHorseRepository::new(state.db.clone());
    let handler = ListHorsesHandler::new(&repository);
    let request = request.unwrap_or(ListHorsesRequest {
        search: None,
        sort_by: None,
        sort_direction: None,
    });

    handler
        .handle(ListHorsesQuery {
            search: request.search,
            sort_by: request.sort_by.unwrap_or_else(|| "name".to_string()),
            sort_direction: request.sort_direction.unwrap_or_else(|| "asc".to_string()),
        })
        .await
}

#[tauri::command]
pub async fn get_horse_details(
    id: String,
    state: State<'_, AppState>,
) -> Result<HorseDetails, String> {
    let repository = SqliteHorseRepository::new(state.db.clone());
    let handler = GetHorseDetailsHandler::new(&repository);

    handler.handle(GetHorseDetailsQuery { id }).await
}
