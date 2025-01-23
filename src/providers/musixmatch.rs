use std::time::Duration;

use super::Provider;
use crate::{
    providers::{Response, SongMetadata},
    song::SongInfo,
};
use anyhow::Result;
use musixmatch_inofficial::models::{SubtitleFormat as MXSubFormat, TrackId as MXTrackId};
use musixmatch_inofficial::Musixmatch as MusixMatchInner;
use tokio::runtime::Runtime as TokioRuntime;

pub struct Musixmatch {
    inner: MusixMatchInner,
    tokio_runtime: TokioRuntime,
}

impl Musixmatch {
    pub fn new() -> Result<Musixmatch> {
        Ok(Musixmatch {
            inner: MusixMatchInner::default(),
            tokio_runtime: TokioRuntime::new()?,
        })
    }
}

impl Provider for Musixmatch {
    fn search(&self, query: &SongInfo) -> Result<Response> {
        let artist_names_str = query.artist_names_str(Default::default()).unwrap();

        let track_query = self.inner.track_search();

        let tracks = self.tokio_runtime.block_on(async {
            if query.has_name_only() {
                track_query.q(query.name())
            } else {
                track_query
                    .q_track(query.name())
                    .q_artist(&artist_names_str)
            }
            .send(5, 1)
            .await
        })?;

        Ok(tracks
            .iter()
            .map(|track| {
                let lyrics = self
                    .tokio_runtime
                    .block_on(self.inner.track_subtitle(
                        MXTrackId::TrackId(track.track_id),
                        MXSubFormat::Lrc,
                        query.duration().map(|duration| duration.as_secs() as f32),
                        Some(1f32),
                    ))
                    .ok();

                let lyrics_ref = lyrics.as_ref();

                // SongMetadata::from((track, lyrics_ref.unwrap()));

                SongMetadata {
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

impl std::fmt::Debug for Musixmatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Musixmatch API")
    }
}
