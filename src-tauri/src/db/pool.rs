use std::collections::HashMap;
use std::time::Duration;

use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use tokio::sync::Mutex;
use uuid::Uuid;

use super::error::DbError;
use crate::profiles::DbConnectionConfig;

const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);

/// Caches one connection pool per profile that has a database connection
/// configured. Pools are created lazily on first use and kept for the
/// app's lifetime; `invalidate` should be called whenever a profile's DB
/// config/password changes or the profile is removed, so a stale pool
/// with old credentials never lingers. Note: invalidating does not cancel
/// any query already in flight against the old pool -- that query simply
/// completes against a connection that will be dropped once idle.
pub struct DbPoolCache {
    pools: Mutex<HashMap<Uuid, sqlx::MySqlPool>>,
}

impl DbPoolCache {
    pub fn new() -> Self {
        Self {
            pools: Mutex::new(HashMap::new()),
        }
    }

    pub async fn get_or_create(
        &self,
        profile_id: Uuid,
        cfg: &DbConnectionConfig,
        password: &str,
    ) -> Result<sqlx::MySqlPool, DbError> {
        let mut pools = self.pools.lock().await;
        if let Some(pool) = pools.get(&profile_id) {
            return Ok(pool.clone());
        }

        let options = MySqlConnectOptions::new()
            .host(&cfg.host)
            .port(cfg.port)
            .username(&cfg.username)
            .password(password)
            .database(&cfg.database);

        let pool = MySqlPoolOptions::new()
            .max_connections(3)
            .acquire_timeout(CONNECT_TIMEOUT)
            .connect_with(options)
            .await
            .map_err(DbError::from)?;

        pools.insert(profile_id, pool.clone());
        Ok(pool)
    }

    pub async fn invalidate(&self, profile_id: Uuid) {
        self.pools.lock().await.remove(&profile_id);
    }
}

impl Default for DbPoolCache {
    fn default() -> Self {
        Self::new()
    }
}
