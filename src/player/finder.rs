// Experimental support for multiple OSes
// Linux/BSD: MPRIS, Windows: GSMTC

// TODO: remove this
#![allow(unused)]

use anyhow::{anyhow, Result};
use gsmtc::ManagerEvent;
use tokio::sync::mpsc::UnboundedReceiver;

use super::{
    iter::{GsmtcEventIter, PlayerIter},
    player::Player,
};

#[derive(Debug)]
pub struct PlayerFinder {
    inner: InnerPlayerFinder,
}

impl PlayerFinder {
    pub fn new() -> Result<PlayerFinder> {
        Ok(PlayerFinder {
            inner: InnerPlayerFinder::new()?,
        })
    }

    pub fn iter_players(&self) -> Result<PlayerIter> {
        self.inner.iter_players()
    }

    pub fn find_all(&self) -> Result<Vec<Player>> {
        self.iter_players()
            .map(|iter| iter.into_iter().flatten().collect())
    }

    pub fn find_first(&self) -> Result<Player> {
        self.iter_players()
            .map(|iter| iter.into_iter().flatten().next().ok_or(anyhow!("")))?
    }

    pub fn find_active(&self) -> Result<Player> {
        self.iter_players().map(|iter| {
            iter.flatten()
                .find(|player| {
                    if let Ok(status) = player.get_playback_status() {
                        status.is_active()
                    } else {
                        false
                    }
                })
                .ok_or(anyhow!(""))
        })?
    }
}

#[derive(Debug)]
enum InnerPlayerFinder {
    Mpris(mpris::PlayerFinder),
    Gsmtc(UnboundedReceiver<ManagerEvent>),
}

impl InnerPlayerFinder {
    pub fn new() -> Result<Self> {
        #[cfg(all(unix, not(target_vendor = "apple")))]
        {
            return Ok(Self::Mpris(mpris::PlayerFinder::new()?));
        }

        #[cfg(windows)]
        {
            return Ok(Self::Gsmtc(gsmtc::SessionManager::create()?));
        }
    }

    pub fn iter_players(&self) -> Result<PlayerIter> {
        Ok(match &self {
            Self::Mpris(mpris) => PlayerIter::Mpris(mpris.iter_players()?),
            Self::Gsmtc(gsmtc) => PlayerIter::Gsmtc(todo!()),
        })
    }
}
