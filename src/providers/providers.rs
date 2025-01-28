use crate::{errors::NoError, song::SongInfo};
use anyhow::{anyhow, Result};
use enum_wrap::{enum_wrap, enum_wrap_impl};

use super::{LRCLib, Musixmatch, Response};

#[enum_wrap_impl]
pub trait ProviderTrait: core::fmt::Debug {
    fn search(&self, query: &SongInfo) -> Result<Response>;
}

pub type ProviderList<'a> = Vec<&'a dyn ProviderTrait>;

#[derive(Debug)]
pub struct LyricsFinder<'a> {
    providers: ProviderList<'a>,
}

impl<'a> LyricsFinder<'a> {
    pub fn new(providers: ProviderList<'a>) -> LyricsFinder<'a> {
        Self { providers }
    }

    pub fn find(&self, query: &SongInfo) -> Result<Response> {
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
