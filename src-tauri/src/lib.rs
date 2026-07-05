mod modules;
mod platform;
mod state;

use modules::horse::interfaces::tauri_commands::{
    archive_horse, create_horse, get_horse_details, list_horses, update_horse,
};
use modules::inventory::interfaces::tauri_commands::{
    archive_inventory_item, create_inventory_item, get_inventory_item_details,
    list_inventory_items, update_inventory_item,
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
            archive_horse,
            archive_inventory_item,
            create_horse,
            create_inventory_item,
            get_horse_details,
            get_inventory_item_details,
            list_horses,
            list_inventory_items,
            update_horse,
            update_inventory_item
        ])
        .run(tauri::generate_context!())
        .expect("error while running StableHub");
}
