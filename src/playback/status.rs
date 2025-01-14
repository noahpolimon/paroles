use gsmtc::PlaybackStatus as GsmtcStatus;
use mpris::PlaybackStatus as MprisStatus;

#[derive(Debug, PartialEq)]
pub enum PlaybackStatus {
    /// Support: Linux, BSD, Windows
    Playing,
    /// Support: Linux, BSD, Windows
    Paused,
    /// Support: Linux, BSD, Windows
    Stopped,
    /// Support: Windows
    Opened,
    /// Support: Windows
    Closed,
    /// Support: Windows
    Changing,
}

impl PlaybackStatus {
    pub fn from_mpris_status(status: &MprisStatus) -> PlaybackStatus {
        match status {
            MprisStatus::Playing => PlaybackStatus::Playing,
            MprisStatus::Paused => PlaybackStatus::Paused,
            MprisStatus::Stopped => PlaybackStatus::Stopped,
        }
    }

    pub fn from_gsmtc_status(status: &GsmtcStatus) -> PlaybackStatus {
        match status {
            GsmtcStatus::Closed => PlaybackStatus::Closed,
            GsmtcStatus::Opened => PlaybackStatus::Opened,
            GsmtcStatus::Changing => PlaybackStatus::Changing,
            GsmtcStatus::Stopped => PlaybackStatus::Stopped,
            GsmtcStatus::Playing => PlaybackStatus::Playing,
            GsmtcStatus::Paused => PlaybackStatus::Paused,
        }
    }

    pub fn is_active(self) -> bool {
        self == PlaybackStatus::Playing
    }
}
