use thiserror::Error;

#[derive(Debug, Error)]
#[error("")]
pub struct NoError;

#[derive(Debug, Error)]
#[error("Error Finding {0}")]
pub struct FindingError(pub String);

// FIXME: implement as a DBusError equivalent to Gsmtc
#[derive(Debug, Error)]
#[error("")]
pub struct GsmtcError;
