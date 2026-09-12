use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Teleport {
    pub id: u32,
    pub name: String,
    pub map: u16,
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    pub orientation: f32,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ItemSummary {
    pub entry: u32,
    pub name: String,
    pub class: u8,
    pub subclass: u8,
    pub quality: u8,
    pub required_level: u8,
    pub item_level: u16,
    pub inventory_type: u8,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemFilter {
    pub name_substring: Option<String>,
    pub class: Option<u8>,
    pub subclass: Option<u8>,
    pub quality_min: Option<u8>,
    pub required_level_max: Option<u8>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CharacterSummary {
    pub guid: u32,
    pub name: String,
    pub race: u8,
    pub class: u8,
    pub level: u8,
    // Raw 0/1 from the `online` column -- kept as u8 rather than bool to
    // avoid depending on sqlx's TINYINT(1)-vs-bool decoding heuristics for
    // a column not necessarily declared with that exact display width.
    pub online: u8,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterFilter {
    pub name_substring: Option<String>,
    pub online_only: bool,
}
