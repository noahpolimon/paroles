use anyhow::anyhow;
use anyhow::Result;
use lrc::Lyrics;
use lyrics::TrackInfo;
use mpris::PlaybackStatus;
use providers::Provider;

mod errors;
mod lyrics;
mod providers;
mod response;

fn main() -> Result<()> {
    let player_finder = mpris::PlayerFinder::new()?;
    // FIXME: find other mpris player with status Playing if lyrics for this one is not found.
    //
    // Reason: applications such as KDEConnect also uses MPRIS, so it might default to that if media
    // such as a YT video or Twitch Stream is playing on other devices.
    let player = player_finder.find_active();
    let mpris_metadata: mpris::Metadata;

    if let Ok(player) = player {
        let metadata_temp = player.get_metadata();

        if let Ok(metadata) = metadata_temp {
            mpris_metadata = metadata;
        } else {
            return Err(anyhow!(""));
        }
    } else {
        return Err(anyhow!(""));
    }

    let artists = mpris_metadata.artists();
    let title = mpris_metadata.title();
    let album = mpris_metadata.album_name();

    if let Some(title) = title {
        let query = TrackInfo::new(
            title.into(),
            artists.map(|artists| artists.iter().map(ToString::to_string).collect()),
            album.map(|a| a.into()),
            mpris_metadata.length(),
        );

        println!(
            "Currently Playing: {}\n",
            query.to_title(Default::default())
        );

        let response = Provider::LRCLib.search(query.clone())?;

        let mut index = 0;

        for lyrics in &response {
            if lyrics.synced_lyrics.is_some() {
                break;
            }

            index += 1;
        }

        let lyrics = if index >= response.len() {
            return Err(anyhow!("Synced Lyrics not found"));
        } else {
            response.get(index).unwrap().synced_lyrics.as_ref()
        };

        let lrc = Lyrics::from_str(lyrics.unwrap());
        let mut player = player_finder.find_active().unwrap();

        // TODO: refactor
        if let Ok(lrc) = lrc {
            for (tag, line) in lrc.get_timed_lines().iter() {
                let mut position = player.get_position().unwrap();

                while tag.get_timestamp() >= position.as_millis().try_into().unwrap() {
                    if !player.is_running() {
                        break;
                    }

                    if player.get_playback_status().unwrap() != PlaybackStatus::Playing {
                        player = player_finder.find_active().unwrap();
                    }

                    position = player.get_position().unwrap();
                }

                println!("{} {}", tag, line);
            }
        } else {
            return Err(anyhow!(""));
        }
    } else {
        return Err(anyhow!(""));
    }

    Ok(())
}
