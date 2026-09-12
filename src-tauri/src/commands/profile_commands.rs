use tauri::State;
use uuid::Uuid;

use crate::error::AppError;
use crate::gm_actions;
use crate::profiles::{self, ProfileStore, ServerProfile};
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
pub fn add_profile(
    store: State<'_, ProfileStore>,
    name: String,
    host: String,
    soap_port: u16,
    username: String,
    password: String,
) -> Result<ServerProfile, String> {
    store.add(name, host, soap_port, username, password)
}

#[tauri::command]
pub fn update_profile(
    store: State<'_, ProfileStore>,
    id: Uuid,
    name: String,
    host: String,
    soap_port: u16,
    username: String,
    password: Option<String>,
) -> Result<ServerProfile, String> {
    store.update(id, name, host, soap_port, username, password)
}

#[tauri::command]
pub fn remove_profile(store: State<'_, ProfileStore>, id: Uuid) -> Result<(), String> {
    store.remove(id)
}

#[tauri::command]
pub async fn test_connection(
    store: State<'_, ProfileStore>,
    id: Uuid,
) -> Result<ConnectionTestResult, AppError> {
    let profile = store
        .get(id)
        .map_err(|e| AppError::new(crate::error::ErrorKind::NotFound, e))?;
    let password = profiles::get_password(id)
        .map_err(|e| AppError::new(crate::error::ErrorKind::KeychainError, e))?;

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
