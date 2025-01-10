use anyhow::{anyhow, Result};
use gsmtc::{ManagerEvent, SessionUpdateEvent};

use tokio::runtime::Runtime as TokioRuntime;

use super::player::Player;

#[derive(Debug)]
pub enum PlayerIter {
    Mpris(mpris::PlayerIter),
    Gsmtc(GsmtcEventIter),
}

impl Iterator for PlayerIter {
    type Item = Result<Player>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            PlayerIter::Mpris(iter) => iter
                .next()
                .map(|result| result.map(Player::Mpris).map_err(|err| anyhow!(err))),
            PlayerIter::Gsmtc(iter) => iter.next().map(|result| result.map(|_| todo!())),
        }
    }
}

#[derive(Debug)]
pub struct GsmtcEventIter {
    manager_events: std::vec::IntoIter<ManagerEvent>,
    tokio_runtime: TokioRuntime,
}

impl GsmtcEventIter {
    pub fn new(events: Vec<ManagerEvent>) -> Result<Self> {
        Ok(Self {
            manager_events: events.into_iter(),
            tokio_runtime: TokioRuntime::new()?,
        })
    }

    // pub fn with_tokio_runtime(events: Vec<ManagerEvent>, runtime: TokioRuntime) -> Self {
    //     Self {
    //         manager_events: events.into_iter(),
    //         tokio_runtime: runtime,
    //     }
    // }
}

impl Iterator for GsmtcEventIter {
    type Item = Result<SessionUpdateEvent>;

    fn next(&mut self) -> Option<Self::Item> {
        let event = self.manager_events.next()?;
        match event {
            ManagerEvent::SessionCreated {
                mut rx,
                session_id: _,
                source: _,
            } => self.tokio_runtime.block_on(async move { todo!() }),
            _ => None,
        }
    }
}
