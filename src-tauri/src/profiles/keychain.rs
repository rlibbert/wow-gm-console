use uuid::Uuid;

const SERVICE_NAME: &str = "wow-gm-console";

fn entry_for(id: Uuid) -> Result<keyring::Entry, String> {
    keyring::Entry::new(SERVICE_NAME, &id.to_string())
        .map_err(|e| format!("failed to access system keychain: {e}"))
}

fn db_entry_for(id: Uuid) -> Result<keyring::Entry, String> {
    keyring::Entry::new(SERVICE_NAME, &format!("{id}-db"))
        .map_err(|e| format!("failed to access system keychain: {e}"))
}

pub fn set_password(id: Uuid, password: &str) -> Result<(), String> {
    entry_for(id)?
        .set_password(password)
        .map_err(|e| format!("failed to store password in system keychain: {e}"))
}

pub fn get_password(id: Uuid) -> Result<String, String> {
    entry_for(id)?
        .get_password()
        .map_err(|e| format!("failed to read password from system keychain: {e}"))
}

pub fn delete_password(id: Uuid) -> Result<(), String> {
    match entry_for(id)?.delete_credential() {
        Ok(()) => Ok(()),
        // Already gone is not an error from the caller's perspective.
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(format!("failed to remove password from system keychain: {e}")),
    }
}

pub fn set_db_password(id: Uuid, password: &str) -> Result<(), String> {
    db_entry_for(id)?
        .set_password(password)
        .map_err(|e| format!("failed to store database password in system keychain: {e}"))
}

pub fn get_db_password(id: Uuid) -> Result<String, String> {
    db_entry_for(id)?
        .get_password()
        .map_err(|e| format!("failed to read database password from system keychain: {e}"))
}

pub fn delete_db_password(id: Uuid) -> Result<(), String> {
    match db_entry_for(id)?.delete_credential() {
        Ok(()) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(format!(
            "failed to remove database password from system keychain: {e}"
        )),
    }
}
