use tauri::State;
use uuid::Uuid;

use super::helpers::run_command;
use crate::error::AppError;
use crate::gm_actions;
use crate::profiles::ProfileStore;

#[tauri::command]
pub async fn gm_revive(
    store: State<'_, ProfileStore>,
    profile_id: Uuid,
    target: Option<String>,
) -> Result<String, AppError> {
    let cmd = gm_actions::revive(target.as_deref());
    run_command(&store, profile_id, &cmd).await
}

#[tauri::command]
pub async fn gm_add_item(
    store: State<'_, ProfileStore>,
    profile_id: Uuid,
    item: String,
    count: u32,
) -> Result<String, AppError> {
    let cmd = gm_actions::add_item(&item, count);
    run_command(&store, profile_id, &cmd).await
}

#[tauri::command]
pub async fn gm_set_level(
    store: State<'_, ProfileStore>,
    profile_id: Uuid,
    target: String,
    level: u8,
) -> Result<String, AppError> {
    let cmd = gm_actions::set_level(&target, level);
    run_command(&store, profile_id, &cmd).await
}

#[tauri::command]
pub async fn gm_set_gold(
    store: State<'_, ProfileStore>,
    profile_id: Uuid,
    amount_copper: u64,
) -> Result<String, AppError> {
    let cmd = gm_actions::set_gold(amount_copper);
    run_command(&store, profile_id, &cmd).await
}

#[tauri::command]
pub async fn gm_teleport_named(
    store: State<'_, ProfileStore>,
    profile_id: Uuid,
    target: String,
    location: String,
) -> Result<String, AppError> {
    let cmd = gm_actions::teleport_named(&target, &location);
    run_command(&store, profile_id, &cmd).await
}

#[tauri::command]
pub async fn gm_teleport_coords(
    store: State<'_, ProfileStore>,
    profile_id: Uuid,
    map: u32,
    x: f32,
    y: f32,
    z: f32,
) -> Result<String, AppError> {
    let cmd = gm_actions::teleport_coords(map, x, y, z);
    run_command(&store, profile_id, &cmd).await
}

#[tauri::command]
pub async fn gm_kick(
    store: State<'_, ProfileStore>,
    profile_id: Uuid,
    target: String,
    reason: Option<String>,
) -> Result<String, AppError> {
    let cmd = gm_actions::kick(&target, reason.as_deref());
    run_command(&store, profile_id, &cmd).await
}

#[tauri::command]
pub async fn gm_ban_account(
    store: State<'_, ProfileStore>,
    profile_id: Uuid,
    target: String,
    duration: String,
    reason: String,
) -> Result<String, AppError> {
    let cmd = gm_actions::ban_account(&target, &duration, &reason);
    run_command(&store, profile_id, &cmd).await
}

#[tauri::command]
pub async fn gm_server_info(
    store: State<'_, ProfileStore>,
    profile_id: Uuid,
) -> Result<String, AppError> {
    let cmd = gm_actions::server_info();
    run_command(&store, profile_id, &cmd).await
}

#[tauri::command]
pub async fn gm_reload_table(
    store: State<'_, ProfileStore>,
    profile_id: Uuid,
    table: String,
) -> Result<String, AppError> {
    let cmd = gm_actions::reload_table(&table);
    run_command(&store, profile_id, &cmd).await
}
