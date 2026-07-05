use std::fs;

use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
use tauri::{AppHandle, Manager};

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./src/platform/sqlite/migrations");

pub async fn connect(app: &AppHandle) -> Result<SqlitePool, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("Nie udało się ustalić katalogu danych aplikacji: {error}"))?;

    fs::create_dir_all(&app_data_dir)
        .map_err(|error| format!("Nie udało się utworzyć katalogu danych aplikacji: {error}"))?;

    let database_path = app_data_dir.join("stablehub.sqlite");
    let connection_options = SqliteConnectOptions::new()
        .filename(database_path)
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connection_options)
        .await
        .map_err(|error| format!("Nie udało się otworzyć bazy SQLite: {error}"))?;

    MIGRATOR
        .run(&pool)
        .await
        .map_err(|error| format!("Nie udało się uruchomić migracji SQLite: {error}"))?;

    Ok(pool)
}
