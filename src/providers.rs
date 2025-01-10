use crate::{
    errors::NoError,
    response::{Response, ResponseError},
    track::{FeatDelimiter, TrackInfo},
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Deserialize, Serialize, EnumIter)]
pub enum Provider {
    LRCLib,
    Genius,
    All,
}

impl Provider {
    pub fn search(&self, query: TrackInfo) -> Result<Response> {
        match self {
            Self::LRCLib => Self::search_lrclib(query),
            Self::Genius => Self::search_genius(query),
            Self::All => Self::search_all(query),
        }
    }

    fn search_all(query: TrackInfo) -> Result<Response> {
        let mut response = vec![];
        let mut err = anyhow!(NoError);

        for provider in Provider::iter().filter(|p| p != &Provider::All) {
            match provider.search(query.clone()) {
                Ok(metadata) => response.extend(metadata),
                Err(e) => {
                    if err.is::<NoError>() {
                        err = e;
                    }
                }
            };
        }

        if response.is_empty() && !err.is::<NoError>() {
            return Err(err);
        }

        Ok(response)
    }

    fn search_lrclib(query: TrackInfo) -> Result<Response> {
        let feat_delim = FeatDelimiter::Comma;

        let mut params = Vec::new();

        let is_conditional_search = if query.has_name_only() {
            params.push(("q", query.name().clone()));

            true
        } else {
            params.push(("track_name", query.name().clone()));

            let artists = query.artist_names_str(feat_delim);

            if let Some(artists) = artists {
                params.push(("artist_name", artists));
            }

            if let Some(album) = query.album_name() {
                params.push(("album_name", album.clone()))
            }

            if let Some(duration) = query.duration() {
                params.push(("duration", duration.as_secs_f64().to_string()))
            }

            false
        };

        let url = reqwest::Url::parse_with_params("https://lrclib.net/api/search", &params)?;
        let res = reqwest::blocking::get(url)?;

        let result = match res.status() {
            reqwest::StatusCode::OK => Ok(res.json::<Response>()?),
            reqwest::StatusCode::BAD_REQUEST
            | reqwest::StatusCode::SERVICE_UNAVAILABLE
            | reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
                Err(res.json::<ResponseError>()?.into())
            }
            _ => Err(ResponseError::new(
                None,
                "UnknownError".into(),
                "Unknown error happened".into(),
            )
            .into()),
        };

        if result.is_err() && !is_conditional_search {
            return Self::search_lrclib(TrackInfo::with_name(query.to_title(feat_delim)));
        }

        result
    }

    fn search_genius(query: TrackInfo) -> Result<Response> {
        todo!()
    }
}
