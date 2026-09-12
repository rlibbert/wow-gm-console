//! Pure builders for AzerothCore GM command strings. No I/O here -- these are
//! unit-tested independently of the network layer, and both the curated
//! action buttons and the raw console ultimately send whatever string these
//! functions produce through the same `soap::execute_command` call.

pub fn revive(target: Option<&str>) -> String {
    match target {
        Some(name) if !name.is_empty() => format!("revive {name}"),
        _ => "revive".to_string(),
    }
}

pub fn add_item(item: &str, count: u32) -> String {
    format!("additem {item} {count}")
}

pub fn set_level(target: &str, level: u8) -> String {
    format!("character level {target} {level}")
}

pub fn set_gold(amount_copper: u64) -> String {
    format!("modify money {amount_copper}")
}

pub fn teleport_named(target: &str, location: &str) -> String {
    format!("tele name {target} {location}")
}

pub fn teleport_coords(map: u32, x: f32, y: f32, z: f32) -> String {
    format!("go xyz {x} {y} {z} {map}")
}

pub fn kick(target: &str, reason: Option<&str>) -> String {
    match reason {
        Some(r) if !r.is_empty() => format!("kick {target} {r}"),
        _ => format!("kick {target}"),
    }
}

pub fn ban_account(target: &str, duration: &str, reason: &str) -> String {
    format!("ban account {target} {duration} {reason}")
}

pub fn server_info() -> String {
    "server info".to_string()
}

pub fn reload_table(table: &str) -> String {
    format!("reload {table}")
}

/// Strips a single leading `.` from a raw command typed by the user, so both
/// `revive Jaarl` and `.revive Jaarl` work identically.
pub fn normalize_raw_command(input: &str) -> String {
    let trimmed = input.trim();
    trimmed.strip_prefix('.').unwrap_or(trimmed).trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn revive_with_and_without_target() {
        assert_eq!(revive(Some("Jaarl")), "revive Jaarl");
        assert_eq!(revive(None), "revive");
        assert_eq!(revive(Some("")), "revive");
    }

    #[test]
    fn add_item_formats_correctly() {
        assert_eq!(add_item("2825", 1), "additem 2825 1");
    }

    #[test]
    fn kick_with_and_without_reason() {
        assert_eq!(kick("Bazzul", Some("AFK")), "kick Bazzul AFK");
        assert_eq!(kick("Bazzul", None), "kick Bazzul");
    }

    #[test]
    fn normalize_strips_leading_dot() {
        assert_eq!(normalize_raw_command(".server info"), "server info");
        assert_eq!(normalize_raw_command("server info"), "server info");
        assert_eq!(normalize_raw_command("  .revive Jaarl  "), "revive Jaarl");
    }
}
