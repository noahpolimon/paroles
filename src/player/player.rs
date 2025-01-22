use std::time::Duration;

use crate::playback::{LoopStatus, PlaybackStatus};
use anyhow::{anyhow, Result};
use gsmtc::SessionUpdateEvent;
use mpris::Player as Mpris;
use tokio::sync::mpsc::UnboundedReceiver;

// TODO: wrap for easier manipulation
type Gsmtc = UnboundedReceiver<SessionUpdateEvent>;

#[derive(Debug)]
pub enum Player {
    Mpris(Mpris),
    Gsmtc(Gsmtc),
}

impl Player {
    pub fn get_playback_status(&self) -> Result<PlaybackStatus> {
        match self {
            Player::Mpris(player) => Ok(player
                .get_playback_status()
                .map_err(|err| anyhow!(err))?
                .into()),
            Player::Gsmtc(_) => todo!(),
        }
    }

    pub fn get_loop_status(&self) -> Result<LoopStatus> {
        match self {
            Player::Mpris(player) => {
                Ok(player.get_loop_status().map_err(|err| anyhow!(err))?.into())
            }
            Player::Gsmtc(_) => todo!(),
        }
    }

    pub fn get_position(&self) -> Result<Duration> {
        match self {
            Player::Mpris(player) => player.get_position().map_err(|err| anyhow!(err)),
            Player::Gsmtc(_) => todo!(),
        }
    }
}
