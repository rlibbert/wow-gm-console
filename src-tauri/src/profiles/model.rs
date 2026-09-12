use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerProfile {
    pub id: Uuid,
    pub name: String,
    pub host: String,
    pub soap_port: u16,
    pub username: String,
    /// Optional read-only database connection, used only to power the
    /// waypoint/item pickers. Absent on profiles saved before this field
    /// existed -- `#[serde(default)]` deserializes those cleanly to `None`.
    #[serde(default)]
    pub db: Option<DbConnectionConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DbConnectionConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
}
