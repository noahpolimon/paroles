use crate::{response::Response, track::TrackInfo};
use anyhow::Result;

use super::Provider;

#[derive(Debug, Default)]
pub struct Genius {}
impl Genius {
    pub fn new() -> Genius {
        todo!()
    }
}
impl Provider for Genius {
    fn search(&self, query: TrackInfo) -> Result<Response> {
        todo!()
    }
}
