use std::time::Duration;

use crate::playback::PlaybackStatus;
use anyhow::{anyhow, Result};
use dbus::arg::RefArg;
use gsmtc::SessionUpdateEvent;
use mpris::Player as Mpris;
use tokio::sync::mpsc::UnboundedReceiver;

// TODO: wrap for eaier manipulation
type Gsmtc = UnboundedReceiver<SessionUpdateEvent>;

#[derive(Debug)]
pub enum Player {
    Mpris(Mpris),
    Gsmtc(Gsmtc),
}

impl Player {
    pub fn get_playback_status(&self) -> Result<PlaybackStatus> {
        match self {
            Player::Mpris(player) => player
                .get_playback_status()
                .map(|status| PlaybackStatus::from_mpris_status(&status))
                .map_err(|err| anyhow!(err)),
            Player::Gsmtc(events) => todo!(),
        }
    }

    pub fn get_position(&self) -> Result<Duration> {
        match self {
            Player::Mpris(player) => player.get_position().map_err(|err| anyhow!(err)),
            Player::Gsmtc(events) => todo!(),
        }
    }
}
