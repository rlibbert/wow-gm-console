use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("could not connect to database: {0}")]
    ConnectionFailed(String),
    #[error("database authentication failed")]
    AuthFailed,
    #[error("database query failed: {0}")]
    QueryFailed(String),
    #[error("database request timed out")]
    Timeout,
}

impl From<sqlx::Error> for DbError {
    fn from(err: sqlx::Error) -> Self {
        match &err {
            sqlx::Error::Database(db_err) => {
                // The generic `DatabaseError` trait only exposes the ANSI
                // SQLSTATE (`code()`, e.g. "28000" for an auth failure) --
                // the MySQL-specific numeric error code (1045 =
                // ER_ACCESS_DENIED_ERROR) needs downcasting to the
                // MySQL-specific error type to read via `.number()`.
                let is_access_denied = db_err
                    .try_downcast_ref::<sqlx::mysql::MySqlDatabaseError>()
                    .map(|e| e.number() == 1045)
                    .unwrap_or(false);
                if is_access_denied {
                    DbError::AuthFailed
                } else {
                    DbError::QueryFailed(db_err.message().to_string())
                }
            }
            sqlx::Error::PoolTimedOut => DbError::Timeout,
            sqlx::Error::Io(_) => DbError::ConnectionFailed(err.to_string()),
            other => DbError::QueryFailed(other.to_string()),
        }
    }
}
