use std::time::Duration;

use anyhow::Result;
use musixmatch_inofficial::models as mx;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SongMetadata {
    pub id: Option<i64>,
    pub track_name: Option<String>,
    pub artist_name: Option<String>,
    pub album_name: Option<String>,
    pub duration: Option<f64>,
    // TODO: remove
    pub instrumental: Option<bool>,
    // TODO: remove
    pub plain_lyrics: Option<String>,
    pub synced_lyrics: Option<String>,
}

impl From<(&mx::Track, &mx::Subtitle)> for SongMetadata {
    fn from(value: (&mx::Track, &mx::Subtitle)) -> SongMetadata {
        let track = value.0;
        let sub = value.1;

        SongMetadata {
            id: Some(sub.subtitle_id as i64),
            artist_name: Some(track.artist_name.clone()),
            track_name: Some(track.track_name.clone()),
            album_name: Some(track.album_name.clone()),
            duration: Some(Duration::from_secs(sub.subtitle_length as u64).as_millis() as f64),
            instrumental: Some(track.instrumental),
            plain_lyrics: if !track.has_richsync {
                Some(sub.subtitle_body.clone())
            } else {
                None
            },
            synced_lyrics: if track.has_richsync {
                Some(sub.subtitle_body.clone())
            } else {
                None
            },
        }
    }
}

pub type Response = Vec<SongMetadata>;

#[derive(Debug, Deserialize, Error)]
#[serde(rename_all = "camelCase")]
#[error("{error}: {message}")]
pub struct ResponseError {
    status_code: u16,
    error: String,
    message: String,
}

impl ResponseError {
    pub fn new(status_code: u16, error: String, message: String) -> ResponseError {
        ResponseError {
            status_code,
            error,
            message,
        }
    }
}
