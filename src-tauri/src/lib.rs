mod commands;
mod error;
mod gm_actions;
mod profiles;
pub mod soap;

use profiles::ProfileStore;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let profile_store = ProfileStore::load().expect("failed to load server profiles");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(profile_store)
        .invoke_handler(tauri::generate_handler![
            commands::list_profiles,
            commands::add_profile,
            commands::update_profile,
            commands::remove_profile,
            commands::test_connection,
            commands::send_raw_command,
            commands::gm_revive,
            commands::gm_add_item,
            commands::gm_set_level,
            commands::gm_set_gold,
            commands::gm_teleport_named,
            commands::gm_teleport_coords,
            commands::gm_kick,
            commands::gm_ban_account,
            commands::gm_server_info,
            commands::gm_reload_table,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
