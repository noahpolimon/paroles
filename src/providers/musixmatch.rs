use core::fmt;
use std::{any, str::FromStr, time::Duration};

use crate::{
    providers::{LyricsMetadata, Response},
    track::TrackInfo,
};
use anyhow::{anyhow, Result};
use musixmatch_inofficial::Musixmatch as MX;
use tokio::runtime::Runtime as TokioRuntime;

#[derive(Default)]
pub struct MusixmatchInner(MX);

impl fmt::Debug for MusixmatchInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MusixMatch")
    }
}

use super::Provider;

#[derive(Debug)]
pub struct Musixmatch {
    inner: MusixmatchInner,
    tokio_runtime: TokioRuntime,
}
impl Musixmatch {
    pub fn new() -> Result<Musixmatch> {
        Ok(Musixmatch {
            inner: MusixmatchInner::default(),
            tokio_runtime: TokioRuntime::new()?,
        })
    }

    fn get_api(&self) -> &MX {
        &self.inner.0
    }
}

impl Provider for Musixmatch {
    fn search(&self, query: &TrackInfo) -> Result<Response> {
        let artist_names_str = query.artist_names_str(Default::default()).unwrap();

        let track_query = self.get_api().track_search();

        let tracks = self.tokio_runtime.block_on(async {
            if query.has_name_only() {
                track_query.q(query.name())
            } else {
                track_query
                    .q_track(query.name())
                    .q_artist(&artist_names_str)
            }
            .send(10, 1)
            .await
        })?;

        Ok(tracks
            .iter()
            .map(|track| {
                let lyrics = self
                    .tokio_runtime
                    .block_on(self.get_api().track_subtitle(
                        musixmatch_inofficial::models::TrackId::TrackId(
                            tracks.first().unwrap().track_id,
                        ),
                        musixmatch_inofficial::models::SubtitleFormat::Lrc,
                        query.duration().map(|duration| duration.as_secs() as f32),
                        Some(1f32),
                    ))
                    .ok();

                let lyrics_ref = lyrics.as_ref();

                LyricsMetadata {
                    id: lyrics_ref.map(|sub| sub.subtitle_id as i64),
                    artist_name: Some(track.artist_name.clone()),
                    track_name: Some(track.track_name.clone()),
                    album_name: Some(track.album_name.clone()),
                    duration: lyrics_ref.map(|sub| {
                        Duration::from_secs(sub.subtitle_length as u64).as_millis() as f64
                    }),

                    instrumental: Some(track.instrumental),
                    plain_lyrics: if !track.has_richsync {
                        lyrics_ref.map(|sub| sub.subtitle_body.clone())
                    } else {
                        None
                    },
                    synced_lyrics: if track.has_richsync {
                        lyrics_ref.map(|sub| sub.subtitle_body.clone())
                    } else {
                        None
                    },
                }
            })
            .collect())
    }
}
