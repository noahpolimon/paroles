// Experimental support for multiple OSes
// Linux/BSD: MPRIS, Windows: GSMTC

use crate::errors::FindingError;
use anyhow::{anyhow, Result};
use gsmtc::ManagerEvent;
use mpris::PlayerFinder as Mpris;
use tokio::sync::mpsc::UnboundedReceiver;

use super::{
    Player, {GsmtcEventIter, PlayerIter},
};

// TODO: wrap in type for easier manipulation
type Gsmtc = UnboundedReceiver<ManagerEvent>;

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
            .map_err(|err| anyhow!(FindingError("players".into())))
    }

    pub fn find_first(&self) -> Result<Player> {
        self.iter_players().map(|iter| {
            iter.into_iter()
                .flatten()
                .next()
                .ok_or(anyhow!(FindingError("first player".into())))
        })?
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
                .ok_or(anyhow!(FindingError("active player".into())))
        })?
    }
}

#[derive(Debug)]
enum InnerPlayerFinder {
    Mpris(Mpris),
    Gsmtc(Gsmtc),
}

impl InnerPlayerFinder {
    pub fn new() -> Result<Self> {
        #[cfg(all(unix, not(target_vendor = "apple")))]
        {
            return Ok(Self::Mpris(Mpris::new()?));
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
