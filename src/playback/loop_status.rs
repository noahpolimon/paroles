use gsmtc::AutoRepeatMode as GsmtcStatus;
use mpris::LoopStatus as MprisStatus;

#[derive(Debug, PartialEq)]
pub enum LoopStatus {
    None,
    Track,
    Playlist,
}

impl Into<LoopStatus> for MprisStatus {
    fn into(self) -> LoopStatus {
        match self {
            MprisStatus::None => LoopStatus::None,
            MprisStatus::Track => LoopStatus::Track,
            MprisStatus::Playlist => LoopStatus::Playlist,
        }
    }
}

impl Into<LoopStatus> for GsmtcStatus {
    fn into(self) -> LoopStatus {
        match self {
            GsmtcStatus::None => LoopStatus::None,
            GsmtcStatus::Track => LoopStatus::Track,
            GsmtcStatus::List => LoopStatus::Playlist,
        }
    }
}
