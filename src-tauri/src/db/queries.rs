use super::error::DbError;
use super::types::{ItemFilter, ItemSummary, Teleport};

/// Escapes the characters that are special inside a MySQL `LIKE` pattern
/// (`%`, `_`, and the escape character itself) so a user's search text is
/// treated as a literal substring, not a wildcard pattern.
fn escape_like(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}

/// Pure -- builds the SQL and bind values for a teleport name search, with
/// no I/O, so it's unit-testable without a database.
pub fn build_teleport_query(name_substring: &str, limit: u32) -> (String, String, u32) {
    let sql = "SELECT id, name, map, position_x, position_y, position_z, orientation \
               FROM game_tele WHERE name LIKE ? ESCAPE '\\\\' ORDER BY name LIMIT ?"
        .to_string();
    let pattern = format!("%{}%", escape_like(name_substring));
    (sql, pattern, limit.clamp(1, 2000))
}

pub async fn search_teleports(
    pool: &sqlx::MySqlPool,
    name_substring: &str,
    limit: u32,
) -> Result<Vec<Teleport>, DbError> {
    let (sql, pattern, clamped) = build_teleport_query(name_substring, limit);
    sqlx::query_as::<_, Teleport>(&sql)
        .bind(pattern)
        .bind(clamped)
        .fetch_all(pool)
        .await
        .map_err(DbError::from)
}

pub async fn list_all_teleports(pool: &sqlx::MySqlPool) -> Result<Vec<Teleport>, DbError> {
    sqlx::query_as::<_, Teleport>(
        "SELECT id, name, map, position_x, position_y, position_z, orientation \
         FROM game_tele ORDER BY name",
    )
    .fetch_all(pool)
    .await
    .map_err(DbError::from)
}

#[derive(Debug, Clone)]
enum ItemQueryParam {
    Str(String),
    U8(u8),
    U32(u32),
}

/// Pure -- builds a dynamic WHERE clause and an ordered list of bind values
/// for item_template filtering, with no I/O, so it's unit-testable without
/// a database. The returned params must be bound in order.
fn build_item_query(filter: &ItemFilter, limit: u32) -> (String, Vec<ItemQueryParam>) {
    let mut clauses = Vec::new();
    let mut params = Vec::new();

    if let Some(ref sub) = filter.name_substring {
        if !sub.is_empty() {
            clauses.push("name LIKE ? ESCAPE '\\\\'");
            params.push(ItemQueryParam::Str(format!("%{}%", escape_like(sub))));
        }
    }
    if let Some(class) = filter.class {
        clauses.push("class = ?");
        params.push(ItemQueryParam::U8(class));
    }
    if let Some(subclass) = filter.subclass {
        clauses.push("subclass = ?");
        params.push(ItemQueryParam::U8(subclass));
    }
    if let Some(q) = filter.quality_min {
        clauses.push("Quality >= ?");
        params.push(ItemQueryParam::U8(q));
    }
    if let Some(lvl) = filter.required_level_max {
        clauses.push("RequiredLevel <= ?");
        params.push(ItemQueryParam::U8(lvl));
    }

    let where_sql = if clauses.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", clauses.join(" AND "))
    };

    let sql = format!(
        "SELECT entry, name, class, subclass, Quality AS quality, \
         RequiredLevel AS required_level, ItemLevel AS item_level, \
         InventoryType AS inventory_type \
         FROM item_template {where_sql} ORDER BY name LIMIT ?"
    );
    params.push(ItemQueryParam::U32(limit.clamp(1, 200)));

    (sql, params)
}

pub async fn search_items(
    pool: &sqlx::MySqlPool,
    filter: &ItemFilter,
    limit: u32,
) -> Result<Vec<ItemSummary>, DbError> {
    let (sql, params) = build_item_query(filter, limit);
    let mut query = sqlx::query_as::<_, ItemSummary>(&sql);
    for param in params {
        query = match param {
            ItemQueryParam::Str(s) => query.bind(s),
            ItemQueryParam::U8(v) => query.bind(v),
            ItemQueryParam::U32(v) => query.bind(v),
        };
    }
    query.fetch_all(pool).await.map_err(DbError::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teleport_query_wraps_pattern_and_clamps_limit() {
        let (sql, pattern, limit) = build_teleport_query("stormwind", 50);
        assert!(sql.contains("game_tele"));
        assert!(sql.contains("LIKE ?"));
        assert_eq!(pattern, "%stormwind%");
        assert_eq!(limit, 50);

        let (_, _, clamped_low) = build_teleport_query("x", 0);
        assert_eq!(clamped_low, 1);
        let (_, _, clamped_high) = build_teleport_query("x", 100_000);
        assert_eq!(clamped_high, 2000);
    }

    #[test]
    fn teleport_query_escapes_like_wildcards() {
        let (_, pattern, _) = build_teleport_query("50%_off\\deal", 10);
        assert_eq!(pattern, "%50\\%\\_off\\\\deal%");
    }

    #[test]
    fn item_query_empty_filter_has_no_where_clause() {
        let filter = ItemFilter::default();
        let (sql, params) = build_item_query(&filter, 100);
        assert!(!sql.contains("WHERE"));
        assert_eq!(params.len(), 1); // just the limit
    }

    #[test]
    fn item_query_combines_all_filters_in_order() {
        let filter = ItemFilter {
            name_substring: Some("bow".to_string()),
            class: Some(2),
            subclass: Some(2),
            quality_min: Some(3),
            required_level_max: Some(40),
        };
        let (sql, params) = build_item_query(&filter, 50);
        assert!(sql.contains("name LIKE ?"));
        assert!(sql.contains("class = ?"));
        assert!(sql.contains("subclass = ?"));
        assert!(sql.contains("Quality >= ?"));
        assert!(sql.contains("RequiredLevel <= ?"));
        // 5 filter params + 1 limit param, in the same order clauses were built
        assert_eq!(params.len(), 6);
        assert!(matches!(params[0], ItemQueryParam::Str(_)));
        assert!(matches!(params[5], ItemQueryParam::U32(50)));
    }

    #[test]
    fn item_query_limit_is_clamped() {
        let filter = ItemFilter::default();
        let (_, params) = build_item_query(&filter, 10_000);
        assert!(matches!(params[0], ItemQueryParam::U32(200)));

        let (_, params) = build_item_query(&filter, 0);
        assert!(matches!(params[0], ItemQueryParam::U32(1)));
    }

    #[test]
    fn item_query_ignores_empty_name_substring() {
        let filter = ItemFilter {
            name_substring: Some(String::new()),
            ..Default::default()
        };
        let (sql, params) = build_item_query(&filter, 50);
        assert!(!sql.contains("WHERE"));
        assert_eq!(params.len(), 1);
    }
}
