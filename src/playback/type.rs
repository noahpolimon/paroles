use gsmtc::AutoRepeatMode as GsmtcStatus;
use mpris::LoopStatus as MprisStatus;

#[derive(Debug, PartialEq)]
pub enum LoopStatus {
    None,
    Track,
    Playlist,
}

impl LoopStatus {
    pub fn from_mpris_status(status: &MprisStatus) -> LoopStatus {
        match status {
            MprisStatus::None => LoopStatus::None,
            MprisStatus::Track => LoopStatus::Track,
            MprisStatus::Playlist => LoopStatus::Playlist,
        }
    }

    pub fn from_gsmtc_status(status: &GsmtcStatus) -> LoopStatus {
        match status {
            GsmtcStatus::None => LoopStatus::None,
            GsmtcStatus::Track => LoopStatus::Track,
            GsmtcStatus::List => LoopStatus::Playlist,
        }
    }
}
