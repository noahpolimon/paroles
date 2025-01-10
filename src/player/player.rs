use std::time::Duration;

use crate::playback::status::PlaybackStatus;
use anyhow::{anyhow, Result};
use dbus::arg::RefArg;
use gsmtc::SessionModel as Gsmtc;
use mpris::Player as Mpris;

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
            Player::Gsmtc(model) => model
                .playback
                .as_ref()
                .map(|playback| PlaybackStatus::from_gsmtc_status(&playback.status))
                .ok_or(anyhow!("")),
        }
    }

    pub fn get_position(&self) -> Result<Duration> {
        match self {
            Player::Mpris(player) => player.get_position().map_err(|err| anyhow!(err)),
            Player::Gsmtc(model) => Ok(Duration::from_millis(
                model
                    .timeline
                    .as_ref()
                    .ok_or(anyhow!(""))?
                    .position
                    .as_u64()
                    .ok_or(anyhow!(""))?,
            )),
        }
    }
}
