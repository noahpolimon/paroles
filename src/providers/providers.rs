use crate::{errors::NoError, track::TrackInfo};
use anyhow::{anyhow, Result};

use super::Response;

pub trait Provider: core::fmt::Debug {
    fn search(&self, query: &TrackInfo) -> Result<Response>;
}

type ProviderList<'a> = Vec<&'a dyn Provider>;

#[derive(Debug)]
pub struct LyricsFinder<'a> {
    providers: ProviderList<'a>,
}

impl<'a> LyricsFinder<'a> {
    pub fn new(providers: ProviderList<'a>) -> LyricsFinder<'a> {
        Self { providers }
    }

    pub fn search(&self, query: &TrackInfo) -> Result<Response> {
        let mut response = vec![];
        let mut err = anyhow!(NoError);

        for provider in &self.providers {
            match provider.search(query) {
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
