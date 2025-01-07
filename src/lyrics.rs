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
    Ampersand,
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
            FeatDelimiter::Ampersand => "&",
            FeatDelimiter::Custom(custom) => custom.trim(),
        };

        write!(f, "{}", s)
    }
}

impl FeatDelimiter<'_> {
    pub fn as_sep(&self) -> String {
        format!("{} ", self.to_string())
    }
}

#[derive(Debug, Default, Clone)]
pub struct TrackInfo {
    name: String,
    artist_names: Option<Vec<String>>,
    album_name: Option<String>,
    duration: Option<Duration>,
}

impl TrackInfo {
    pub fn new(
        name: String,
        artist_names: Option<Vec<String>>,
        album_name: Option<String>,
        duration: Option<Duration>,
    ) -> TrackInfo {
        TrackInfo {
            name,
            artist_names,
            album_name,
            duration,
        }
    }

    pub fn with_name(name: String) -> TrackInfo {
        TrackInfo {
            name,
            ..Default::default()
        }
    }

    pub fn from_title(title: String) -> TrackInfo {
        if let Some((artists, name)) = title.split_once("-") {
            TrackInfo {
                artist_names: Some(
                    artists
                        .split(FeatDelimiter::Comma.as_sep().as_str())
                        .map(|split| split.trim().into())
                        .collect(),
                ),
                name: name.to_string(),
                ..Default::default()
            }
        } else {
            TrackInfo::with_name(title)
        }
    }

    pub fn artist_names(&self) -> Option<&Vec<String>> {
        self.artist_names.as_ref()
    }

    pub fn artist_names_str(&self, feat_delim: FeatDelimiter) -> Option<String> {
        let s = if let Some(artists) = &self.artist_names {
            artists.join(feat_delim.as_sep().as_str())
        } else {
            "".into()
        };

        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn duration(&self) -> Option<&Duration> {
        self.duration.as_ref()
    }

    pub fn album_name(&self) -> Option<&String> {
        self.album_name.as_ref()
    }

    pub fn to_title(&self, feat_delim: FeatDelimiter) -> String {
        let artists = self.artist_names_str(feat_delim);

        if let Some(artists) = artists {
            format!("{} - {}", artists, self.name)
        } else {
            self.name.clone()
        }
    }

    pub fn has_name_only(&self) -> bool {
        let has_artist = self.artist_names.is_some() && self.artist_names().unwrap().is_empty();

        !has_artist && self.album_name.is_none() && self.duration.is_none()
    }
}
