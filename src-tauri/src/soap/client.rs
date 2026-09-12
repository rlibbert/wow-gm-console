use std::time::Duration;

use thiserror::Error;

use super::envelope::{build_envelope, parse_response, ParsedResponse};

#[derive(Debug, Error)]
pub enum SoapError {
    #[error("authentication failed")]
    AuthFailed,
    #[error("insufficient privilege")]
    InsufficientPrivilege,
    #[error("request timed out")]
    Timeout,
    #[error("http error: {0}")]
    HttpError(u16),
    #[error("network error: {0}")]
    Network(String),
    #[error("{0}")]
    CommandFailed(String),
    #[error("malformed response: {0}")]
    MalformedResponse(String),
}

const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const READ_TIMEOUT: Duration = Duration::from_secs(30);

/// Sends a single GM command to an AzerothCore worldserver's SOAP interface
/// and returns the raw console output text on success.
pub async fn execute_command(
    host: &str,
    port: u16,
    username: &str,
    password: &str,
    command: &str,
) -> Result<String, SoapError> {
    let url = format!("http://{host}:{port}/");
    let body = build_envelope(command);

    let client = reqwest::Client::builder()
        .connect_timeout(CONNECT_TIMEOUT)
        .timeout(READ_TIMEOUT)
        .build()
        .map_err(|e| SoapError::Network(e.to_string()))?;

    let response = client
        .post(&url)
        .basic_auth(username, Some(password))
        .header("Content-Type", "text/xml; charset=utf-8")
        .header("SOAPAction", "\"urn:AC#executeCommand\"")
        .body(body)
        .send()
        .await
        .map_err(|e| {
            if e.is_timeout() {
                SoapError::Timeout
            } else {
                SoapError::Network(e.to_string())
            }
        })?;

    let status = response.status();

    if status.as_u16() == 401 {
        return Err(SoapError::AuthFailed);
    }
    if status.as_u16() == 403 {
        return Err(SoapError::InsufficientPrivilege);
    }

    let text = response
        .text()
        .await
        .map_err(|e| SoapError::Network(e.to_string()))?;

    // gSOAP typically returns 500 with a Fault body on command failure, but
    // some AzerothCore builds return 200 with a Fault body instead -- always
    // attempt to parse the body regardless of status before treating a
    // non-2xx status as a bare HTTP error.
    match parse_response(&text) {
        Ok(ParsedResponse::Result(result)) => Ok(result),
        Ok(ParsedResponse::Fault(message)) => Err(SoapError::CommandFailed(message)),
        Err(parse_err) => {
            if status.is_success() {
                Err(SoapError::MalformedResponse(parse_err))
            } else {
                Err(SoapError::HttpError(status.as_u16()))
            }
        }
    }
}
