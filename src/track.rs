use anyhow::anyhow;
use std::{fmt, time::Duration};

#[derive(Debug, Default, Clone, Copy)]
pub enum ArtistsDelimiter<'a> {
    #[default]
    Comma,
    X,
    Feat,
    Ft,
    Ampersand,
    Custom(&'a str),
}

impl fmt::Display for ArtistsDelimiter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Comma => ",",
            Self::X => "x",
            Self::Feat => "feat.",
            Self::Ft => "ft.",
            Self::Ampersand => "&",
            Self::Custom(custom) => custom.trim(),
        };

        write!(f, "{}", s)
    }
}

impl ArtistsDelimiter<'_> {
    pub fn to_sep(&self) -> String {
        format!("{} ", self.to_string())
    }
}

#[derive(Debug, Default, Clone)]
pub struct TrackInfo {
    name: String,
    artist_names: Option<Vec<String>>,
    album_name: Option<String>,
    duration: Option<Duration>,
}

impl TrackInfo {
    pub fn new(
        name: String,
        artist_names: Option<Vec<String>>,
        album_name: Option<String>,
        duration: Option<Duration>,
    ) -> TrackInfo {
        TrackInfo {
            name,
            artist_names,
            album_name,
            duration,
        }
    }

    pub fn with_name(name: String) -> TrackInfo {
        TrackInfo {
            name,
            ..Default::default()
        }
    }

    pub fn from_full_title(title: String) -> TrackInfo {
        if let Some((artists, name)) = title.split_once("-") {
            TrackInfo {
                artist_names: Some(
                    artists
                        .split(ArtistsDelimiter::default().to_sep().as_str())
                        .map(|split| split.trim().into())
                        .collect(),
                ),
                name: name.to_string(),
                ..Default::default()
            }
        } else {
            TrackInfo::with_name(title)
        }
    }

    pub fn artist_names(&self) -> Option<&[String]> {
        self.artist_names.as_ref().map(|vec| vec.as_slice())
    }

    pub fn artist_names_str(&self, feat_delim: ArtistsDelimiter) -> Option<String> {
        let s = if let Some(artists) = &self.artist_names {
            artists.join(feat_delim.to_sep().as_str())
        } else {
            "".into()
        };

        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn duration(&self) -> Option<&Duration> {
        self.duration.as_ref()
    }

    pub fn album_name(&self) -> Option<&str> {
        self.album_name.as_deref()
    }

    pub fn to_title(&self, feat_delim: ArtistsDelimiter) -> String {
        let artists = self.artist_names_str(feat_delim);

        if let Some(artists) = artists {
            format!("{} - {}", artists, self.name)
        } else {
            self.name.clone()
        }
    }

    pub fn has_name_only(&self) -> bool {
        let has_artist = self.artist_names.is_some() && self.artist_names().unwrap().is_empty();

        !has_artist && self.album_name.is_none() && self.duration.is_none()
    }
}

impl TryFrom<mpris::Metadata> for TrackInfo {
    type Error = anyhow::Error;

    fn try_from(value: mpris::Metadata) -> Result<Self, Self::Error> {
        Ok(TrackInfo {
            name: value.title().ok_or_else(|| anyhow!(""))?.into(),
            artist_names: value
                .artists()
                .map(|v| v.iter().map(ToString::to_string).collect()),
            album_name: value.album_name().map(ToString::to_string),
            duration: value.length(),
        })
    }
}

impl TryFrom<gsmtc::SessionModel> for TrackInfo {
    type Error = anyhow::Error;

    fn try_from(value: gsmtc::SessionModel) -> Result<Self, Self::Error> {
        let media = value.media.as_ref();
        Ok(TrackInfo {
            name: media.ok_or_else(|| anyhow!(""))?.title.clone(),
            artist_names: media.map(|model| vec![model.artist.clone()]),
            album_name: media
                .map(|model| model.album.as_ref().map(|album| album.title.clone()))
                .flatten(),
            // FIXME: docs for this is unclear to me
            duration: value
                .timeline
                .map(|timeline| Duration::from_millis(timeline.end as u64)),
        })
    }
}
