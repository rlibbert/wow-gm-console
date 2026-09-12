use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppError {
    pub kind: ErrorKind,
    pub message: String,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ErrorKind {
    AuthFailed,
    InsufficientPrivilege,
    NetworkError,
    Timeout,
    CommandFailed,
    MalformedResponse,
    NotFound,
    KeychainError,
    StoreError,
}

impl AppError {
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
}

impl From<crate::soap::SoapError> for AppError {
    fn from(err: crate::soap::SoapError) -> Self {
        use crate::soap::SoapError as S;
        match err {
            S::AuthFailed => AppError::new(ErrorKind::AuthFailed, "Invalid username or password"),
            S::InsufficientPrivilege => AppError::new(
                ErrorKind::InsufficientPrivilege,
                "Account does not have GM level 3 (Administrator) or higher",
            ),
            S::Timeout => AppError::new(ErrorKind::Timeout, "Request timed out"),
            S::HttpError(status) => {
                AppError::new(ErrorKind::NetworkError, format!("HTTP error: {status}"))
            }
            S::Network(msg) => AppError::new(ErrorKind::NetworkError, msg),
            S::CommandFailed(msg) => AppError::new(ErrorKind::CommandFailed, msg),
            S::MalformedResponse(msg) => AppError::new(ErrorKind::MalformedResponse, msg),
        }
    }
}
