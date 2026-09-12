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
}
