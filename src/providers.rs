// TODO: remove this
#![allow(unused)]

use std::time::Duration;

use crate::{
    errors::NoError,
    lyrics::{FeatDelimiter, Query},
    response::{Response, ResponseError},
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
    pub fn search(&self, query: Query) -> Result<Response> {
        match self {
            Provider::LRCLib => LRCLib::search(query),
            Provider::Genius => Genius::search(query),
            Provider::All => {
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
        }
    }
}

pub trait ProviderTrait {
    fn search(query: Query) -> Result<Response>;
}

struct LRCLib;

impl ProviderTrait for LRCLib {
    fn search(query: Query) -> Result<Response> {
        let feat_delim = FeatDelimiter::Comma;

        let mut params = Vec::new();

        let is_conditional_search = if query.has_track_name_only() {
            params.push(("q", query.track_name().clone()));

            true
        } else {
            params.push(("track_name", query.track_name().clone()));

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
            return Self::search(Query::with_track_name(query.to_track_title(feat_delim)));
        }

        result
    }
}

struct Genius;

impl ProviderTrait for Genius {
    fn search(query: Query) -> Result<Response> {
        todo!()
    }
}
