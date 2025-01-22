use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LyricsMetadata {
    pub id: Option<i64>,
    pub track_name: Option<String>,
    pub artist_name: Option<String>,
    pub album_name: Option<String>,
    pub duration: Option<f64>,
    pub instrumental: Option<bool>,
    pub plain_lyrics: Option<String>,
    pub synced_lyrics: Option<String>,
}

pub type Response = Vec<LyricsMetadata>;

#[derive(Debug, Deserialize, Error)]
#[serde(rename_all = "camelCase")]
#[error("{error}: {message}")]
pub struct ResponseError {
    status_code: Option<u16>,
    error: String,
    message: String,
}

impl ResponseError {
    pub fn new(status_code: Option<u16>, error: String, message: String) -> ResponseError {
        ResponseError {
            status_code,
            error,
            message,
        }
    }
}

pub fn req_response_to_local_response(res: reqwest::blocking::Response) -> Result<Response> {
    match res.status() {
        reqwest::StatusCode::OK => Ok(res.json::<Response>()?),
        reqwest::StatusCode::BAD_REQUEST
        | reqwest::StatusCode::SERVICE_UNAVAILABLE
        | reqwest::StatusCode::INTERNAL_SERVER_ERROR => Err(res.json::<ResponseError>()?.into()),
        _ => Err(
            ResponseError::new(None, "UnknownError".into(), "Unknown error happened".into()).into(),
        ),
    }
}
