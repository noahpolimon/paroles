use anyhow::{anyhow, Result};
use reqwest::{blocking::Client, header::HeaderMap};

use crate::{
    response::{req_response_to_local_response, Response},
    track::{FeatDelimiter, TrackInfo},
};

use super::Provider;

#[derive(Debug, Default)]
pub struct LRCLib {
    request_client: Client,
}
impl LRCLib {
    pub fn new() -> Result<LRCLib> {
        let mut headers = HeaderMap::new();
        // headers.insert(
        //     HeaderName::from_str("lrclib-client")?,
        //     HeaderValue::from_str("")?,
        // );
        Ok(Self {
            request_client: Client::builder()
                .default_headers(headers)
                .build()
                .map_err(|err| anyhow!(err))?,
        })
    }
}

impl Provider for LRCLib {
    fn search(&self, query: TrackInfo) -> Result<Response> {
        let feat_delim = FeatDelimiter::Comma;

        let mut params = Vec::new();

        let is_conditional_search = if query.has_name_only() {
            params.push(("q", query.name().into()));

            true
        } else {
            params.push(("track_name", query.name().into()));

            let artists = query.artist_names_str(feat_delim);

            if let Some(artists) = artists {
                params.push(("artist_name", artists));
            }

            if let Some(album) = query.album_name() {
                params.push(("album_name", album.into()))
            }

            if let Some(duration) = query.duration() {
                params.push(("duration", duration.as_secs_f64().to_string()))
            }

            false
        };

        let url = reqwest::Url::parse_with_params("https://lrclib.net/api/search", &params)?;
        let res = self.request_client.get(url).send()?;

        let result = req_response_to_local_response(res);

        if result.is_err() && !is_conditional_search {
            return self.search(TrackInfo::with_name(query.to_title(feat_delim)));
        }

        result
    }
}
