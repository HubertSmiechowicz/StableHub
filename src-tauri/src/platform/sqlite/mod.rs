use std::fs;

use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
use tauri::{AppHandle, Manager};

static HORSE_MIGRATOR: sqlx::migrate::Migrator =
    sqlx::migrate!("./src/modules/horse/infrastructure/persistence/sqlite/migrations");

pub async fn connect(app: &AppHandle) -> Result<SqlitePool, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("Could not resolve app data directory: {error}"))?;

    fs::create_dir_all(&app_data_dir)
        .map_err(|error| format!("Could not create app data directory: {error}"))?;

    let database_path = app_data_dir.join("stablehub.sqlite");
    let connection_options = SqliteConnectOptions::new()
        .filename(database_path)
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connection_options)
        .await
        .map_err(|error| format!("Could not open SQLite database: {error}"))?;

    HORSE_MIGRATOR
        .run(&pool)
        .await
        .map_err(|error| format!("Could not run SQLite migrations: {error}"))?;

    Ok(pool)
}
