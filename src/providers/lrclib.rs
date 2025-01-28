use anyhow::{anyhow, Result};
use reqwest::{blocking::Client, header::HeaderMap};

use crate::song::{ArtistsDelimiter, SongInfo};

use super::{ProviderTrait, Response, ResponseError};

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

    fn req_response_to_local_response(res: reqwest::blocking::Response) -> Result<Response> {
        match res.status() {
            reqwest::StatusCode::OK => Ok(res.json::<Response>()?),
            reqwest::StatusCode::BAD_REQUEST
            | reqwest::StatusCode::SERVICE_UNAVAILABLE
            | reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
                Err(res.json::<ResponseError>()?.into())
            }
            code => Err(ResponseError::new(
                code.as_u16(),
                "UnknownError".into(),
                "Unknown error happened".into(),
            )
            .into()),
        }
    }
}

impl ProviderTrait for LRCLib {
    fn search(&self, query: &SongInfo) -> Result<Response> {
        let feat_delim = ArtistsDelimiter::Comma;

        let mut params = vec![];

        let is_conditional_search = if query.has_title_only() {
            params.push(("q", query.title().into()));

            true
        } else {
            params.push(("track_name", query.title().into()));

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

        let result = Self::req_response_to_local_response(res);

        // let search_again = if let Ok(result) = &result {
        //     result.is_empty()
        // } else {
        //     !is_conditional_search
        // };

        // if search_again {
        //     return self
        //         .search(&TrackInfo::with_name(query.to_title(feat_delim)))
        //         .or_else(|_| self.search(&TrackInfo::with_name(query.name().into())));
        // }

        result
    }
}
