mod modules;
mod platform;
mod state;

use modules::calendar::interfaces::tauri_commands::{
    archive_calendar_entry, create_calendar_entry, get_calendar_entry_details, list_calendar_items,
    update_calendar_entry,
};
use modules::health::interfaces::tauri_commands::{
    archive_health_event, create_health_event, get_health_event_details, list_health_events,
    update_health_event,
};
use modules::horse::interfaces::tauri_commands::{
    archive_horse, create_horse, get_horse_details, list_horses, update_horse,
};
use modules::inventory::interfaces::tauri_commands::{
    apply_inventory_usage, archive_inventory_item, create_inventory_item,
    get_inventory_item_details, list_inventory_items, record_inventory_stocktake,
    register_inventory_delivery, update_inventory_item,
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
            apply_inventory_usage,
            archive_calendar_entry,
            archive_health_event,
            archive_horse,
            archive_inventory_item,
            create_calendar_entry,
            create_health_event,
            create_horse,
            create_inventory_item,
            get_calendar_entry_details,
            get_health_event_details,
            get_horse_details,
            get_inventory_item_details,
            list_calendar_items,
            list_health_events,
            list_horses,
            list_inventory_items,
            record_inventory_stocktake,
            register_inventory_delivery,
            update_calendar_entry,
            update_health_event,
            update_horse,
            update_inventory_item
        ])
        .run(tauri::generate_context!())
        .expect("error while running StableHub");
}
