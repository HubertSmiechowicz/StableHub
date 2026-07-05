use serde::Deserialize;
use tauri::State;

use crate::{
    modules::horse::{
        application::{
            commands::create_horse::{CreateHorseCommand, CreateHorseHandler},
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
pub async fn list_horses(state: State<'_, AppState>) -> Result<Vec<HorseSummary>, String> {
    let repository = SqliteHorseRepository::new(state.db.clone());
    let handler = ListHorsesHandler::new(&repository);

    handler.handle(ListHorsesQuery).await
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
