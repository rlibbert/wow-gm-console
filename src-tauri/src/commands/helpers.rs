use uuid::Uuid;

use crate::error::{AppError, ErrorKind};
use crate::profiles::{self, ProfileStore};
use crate::soap;

/// Resolves a profile by id, fetches its password from the keychain, and
/// sends the given already-built command string over SOAP. This is the one
/// path both the raw console and every curated GM action funnel through, so
/// they share identical error handling.
pub async fn run_command(
    store: &ProfileStore,
    profile_id: Uuid,
    command: &str,
) -> Result<String, AppError> {
    let profile = store
        .get(profile_id)
        .map_err(|e| AppError::new(ErrorKind::NotFound, e))?;
    let password = profiles::get_password(profile_id)
        .map_err(|e| AppError::new(ErrorKind::KeychainError, e))?;

    soap::execute_command(
        &profile.host,
        profile.soap_port,
        &profile.username,
        &password,
        command,
    )
    .await
    .map_err(AppError::from)
}
