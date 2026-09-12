use tauri::State;
use uuid::Uuid;

use super::helpers::run_command;
use crate::error::AppError;
use crate::gm_actions::normalize_raw_command;
use crate::profiles::ProfileStore;

#[tauri::command]
pub async fn send_raw_command(
    store: State<'_, ProfileStore>,
    profile_id: Uuid,
    command: String,
) -> Result<String, AppError> {
    let normalized = normalize_raw_command(&command);
    run_command(&store, profile_id, &normalized).await
}
