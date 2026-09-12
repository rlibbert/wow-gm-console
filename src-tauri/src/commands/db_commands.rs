use serde::Serialize;
use tauri::State;
use uuid::Uuid;

use crate::db::{self, CharacterFilter, CharacterSummary, DbPoolCache, ItemFilter, ItemSummary, Teleport};
use crate::error::{AppError, ErrorKind};
use crate::profiles::{self, ProfileStore};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DbConnectionTestResult {
    pub success: bool,
    pub latency_ms: u128,
    pub message: String,
}

async fn resolve_pool(
    store: &ProfileStore,
    pools: &DbPoolCache,
    id: Uuid,
) -> Result<sqlx::MySqlPool, AppError> {
    let profile = store
        .get(id)
        .map_err(|e| AppError::new(ErrorKind::NotFound, e))?;
    let cfg = profile.db.ok_or_else(|| {
        AppError::new(
            ErrorKind::NotFound,
            "No database connection configured for this profile",
        )
    })?;
    let password = profiles::get_db_password(id)
        .map_err(|e| AppError::new(ErrorKind::KeychainError, e))?;

    pools
        .get_or_create(id, &cfg, &password)
        .await
        .map_err(AppError::from)
}

#[tauri::command]
pub async fn test_db_connection(
    store: State<'_, ProfileStore>,
    pools: State<'_, DbPoolCache>,
    id: Uuid,
) -> Result<DbConnectionTestResult, AppError> {
    let started = std::time::Instant::now();
    let pool = resolve_pool(&store, &pools, id).await?;
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .map_err(|e| AppError::from(db::DbError::from(e)))?;

    Ok(DbConnectionTestResult {
        success: true,
        latency_ms: started.elapsed().as_millis(),
        message: "Connected".to_string(),
    })
}

#[tauri::command]
pub async fn list_teleports(
    store: State<'_, ProfileStore>,
    pools: State<'_, DbPoolCache>,
    id: Uuid,
) -> Result<Vec<Teleport>, AppError> {
    let pool = resolve_pool(&store, &pools, id).await?;
    db::list_all_teleports(&pool).await.map_err(AppError::from)
}

#[tauri::command]
pub async fn search_items(
    store: State<'_, ProfileStore>,
    pools: State<'_, DbPoolCache>,
    id: Uuid,
    filter: ItemFilter,
    limit: Option<u32>,
) -> Result<Vec<ItemSummary>, AppError> {
    let pool = resolve_pool(&store, &pools, id).await?;
    db::search_items(&pool, &filter, limit.unwrap_or(100))
        .await
        .map_err(AppError::from)
}

#[tauri::command]
pub async fn search_characters(
    store: State<'_, ProfileStore>,
    pools: State<'_, DbPoolCache>,
    id: Uuid,
    filter: CharacterFilter,
    limit: Option<u32>,
) -> Result<Vec<CharacterSummary>, AppError> {
    let pool = resolve_pool(&store, &pools, id).await?;
    db::search_characters(&pool, &filter, limit.unwrap_or(100))
        .await
        .map_err(AppError::from)
}
