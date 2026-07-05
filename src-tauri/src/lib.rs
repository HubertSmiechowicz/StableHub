mod modules;
mod platform;
mod state;

use modules::horse::interfaces::tauri_commands::{
    create_horse, get_horse_details, list_horses,
};
use state::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let db = tauri::async_runtime::block_on(platform::sqlite::connect(app.handle()))?;
            app.manage(AppState { db });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_horse,
            get_horse_details,
            list_horses
        ])
        .run(tauri::generate_context!())
        .expect("error while running StableHub");
}
