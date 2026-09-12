use tauri::State;
use uuid::Uuid;

use crate::db::DbPoolCache;
use crate::error::{AppError, ErrorKind};
use crate::gm_actions;
use crate::profiles::{self, DbConnectionConfig, ProfileStore, ServerProfile};
use crate::soap;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionTestResult {
    pub success: bool,
    pub latency_ms: u128,
    pub message: String,
}

#[tauri::command]
pub fn list_profiles(store: State<'_, ProfileStore>) -> Vec<ServerProfile> {
    store.list()
}

#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub fn add_profile(
    store: State<'_, ProfileStore>,
    name: String,
    host: String,
    soap_port: u16,
    username: String,
    password: String,
    db: Option<DbConnectionConfig>,
    db_password: Option<String>,
) -> Result<ServerProfile, AppError> {
    store
        .add(name, host, soap_port, username, password, db, db_password)
        .map_err(|e| AppError::new(ErrorKind::StoreError, e))
}

#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub async fn update_profile(
    store: State<'_, ProfileStore>,
    pools: State<'_, DbPoolCache>,
    id: Uuid,
    name: String,
    host: String,
    soap_port: u16,
    username: String,
    password: Option<String>,
    db: Option<DbConnectionConfig>,
    db_password: Option<String>,
) -> Result<ServerProfile, AppError> {
    let updated = store
        .update(id, name, host, soap_port, username, password, db, db_password)
        .map_err(|e| AppError::new(ErrorKind::StoreError, e))?;
    // Always drop any cached DB pool for this profile so stale
    // host/credentials never linger, regardless of whether the DB fields
    // specifically changed -- pool creation is lazy and cheap to redo.
    pools.invalidate(id).await;
    Ok(updated)
}

#[tauri::command]
pub async fn remove_profile(
    store: State<'_, ProfileStore>,
    pools: State<'_, DbPoolCache>,
    id: Uuid,
) -> Result<(), AppError> {
    store
        .remove(id)
        .map_err(|e| AppError::new(ErrorKind::StoreError, e))?;
    pools.invalidate(id).await;
    Ok(())
}

#[tauri::command]
pub async fn test_connection(
    store: State<'_, ProfileStore>,
    id: Uuid,
) -> Result<ConnectionTestResult, AppError> {
    let profile = store
        .get(id)
        .map_err(|e| AppError::new(ErrorKind::NotFound, e))?;
    let password = profiles::get_password(id)
        .map_err(|e| AppError::new(ErrorKind::KeychainError, e))?;

    let started = std::time::Instant::now();
    let result = soap::execute_command(
        &profile.host,
        profile.soap_port,
        &profile.username,
        &password,
        &gm_actions::server_info(),
    )
    .await;
    let latency_ms = started.elapsed().as_millis();

    match result {
        Ok(message) => Ok(ConnectionTestResult {
            success: true,
            latency_ms,
            message,
        }),
        Err(e) => Err(AppError::from(e)),
    }
}
