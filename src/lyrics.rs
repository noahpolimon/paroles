// TODO: remove this
#![allow(unused)]

use std::{fmt, time::Duration};

use crate::{
    providers::Provider,
    response::{Response, ResponseError},
};
use anyhow::{anyhow, Result};
use serde::Serialize;

#[derive(Default, Clone, Copy)]
pub enum FeatDelimiter<'a> {
    #[default]
    Comma,
    X,
    Feat,
    Ft,
    Featuring,
    Custom(&'a str),
}

impl fmt::Display for FeatDelimiter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            FeatDelimiter::Comma => ",",
            FeatDelimiter::X => "x",
            FeatDelimiter::Feat => "feat.",
            FeatDelimiter::Ft => "ft.",
            FeatDelimiter::Featuring => "featuring",
            FeatDelimiter::Custom(custom) => custom.trim(),
        };

        write!(f, "{} ", s)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Query {
    track_name: String,
    artist_names: Option<Vec<String>>,
    album_name: Option<String>,
    duration: Option<Duration>,
}

impl Query {
    pub fn new(
        track_name: String,
        artist_names: Option<Vec<String>>,
        album_name: Option<String>,
        duration: Option<Duration>,
    ) -> Query {
        Query {
            track_name,
            artist_names,
            album_name,
            duration,
        }
    }

    pub fn with_track_name(track_name: String) -> Query {
        Query {
            track_name,
            ..Default::default()
        }
    }

    pub fn from_title(title: String) -> Query {
        if let Some(title) = title.split_once("-") {
            Query {
                artist_names: Some(
                    title
                        .1
                        .split(FeatDelimiter::Comma.to_string().as_str())
                        .map(ToString::to_string)
                        .collect(),
                ),
                track_name: title.0.to_string(),
                ..Default::default()
            }
        } else {
            Query::with_track_name(title)
        }
    }

    pub fn artist_names(&self) -> Option<&Vec<String>> {
        self.artist_names.as_ref()
    }

    pub fn artist_names_str(&self, feat_delim: FeatDelimiter) -> Option<String> {
        let s = if let Some(artists) = &self.artist_names {
            artists.join(feat_delim.to_string().as_str())
        } else {
            "".into()
        };

        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    }

    pub fn track_name(&self) -> &String {
        &self.track_name
    }

    pub fn duration(&self) -> Option<&Duration> {
        self.duration.as_ref()
    }

    pub fn album_name(&self) -> Option<&String> {
        self.album_name.as_ref()
    }

    pub fn to_track_title(&self, feat_delim: FeatDelimiter) -> String {
        let artists = self.artist_names_str(feat_delim);

        if let Some(artists) = artists {
            format!("{} - {}", artists, self.track_name)
        } else {
            self.track_name.clone()
        }
    }

    pub fn has_track_name_only(&self) -> bool {
        let has_artist = self.artist_names.is_some() && self.artist_names().unwrap().is_empty();

        !has_artist && self.album_name.is_none() && self.duration.is_none()
    }
}
