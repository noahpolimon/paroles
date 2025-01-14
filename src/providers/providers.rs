use crate::{errors::NoError, response::Response, track::TrackInfo};
use anyhow::{anyhow, Result};

pub trait Provider {
    fn search(&self, query: TrackInfo) -> Result<Response>;
}

type LyricsProviders<'a> = Vec<&'a dyn Provider>;

#[macro_export]
macro_rules! provider_list {
    [$($p:expr), +] => {
        {
            let mut seen = vec![];
            let providers: Vec<Option<&dyn Provider>> = vec![$({
                let s = stringify!($p);
                if !seen.contains(&s) {
                    seen.push(s);
                    Some($p)
                } else {
                    None
                }
            }
            ), +];

            providers.into_iter().flatten().collect()
        }
    };
    [$($p:expr,) +] => {
        provider_list![$($p), +]
    };
}

#[macro_export]
macro_rules! lyrics_finder {
    ($($p:expr), +) => {
        LyricsFinder::new(provider_list![$($p), +])
    };
    ($($p:expr,) +) => {
        lyrics_finder![$($p), +]
    };
}

pub struct LyricsFinder<'a> {
    providers: LyricsProviders<'a>,
}

impl<'a> LyricsFinder<'a> {
    pub fn new(providers: LyricsProviders<'a>) -> LyricsFinder<'a> {
        Self { providers }
    }

    pub fn search(&self, query: TrackInfo) -> Result<Response> {
        let mut response = vec![];
        let mut err = anyhow!(NoError);

        for provider in &self.providers {
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
