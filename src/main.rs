// Paroles is a rust-based cli tool and service to fetch synced lyrics and
// synchronize them with playing media.
// Copyright (C) 2025 noahpolimon
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
//
mod errors;
mod events;
mod playback;
mod player;
mod providers;
mod response;
mod track;

use anyhow::anyhow;
use anyhow::Result;
use dbus::ffidisp::Connection;
use lrc::Lyrics;
use lrc::TimeTag;
use providers::LRCLib;
use providers::{LyricsFinder, Provider};
use track::TrackInfo;

fn main() -> Result<()> {
    let connection = Connection::new_session()?;
    let player_finder = mpris::PlayerFinder::for_connection(connection);
    // FIXME: find other mpris player with status Playing if lyrics for this one is not found.
    //
    // Reason: applications such as KDEConnect also uses MPRIS, so it might default to that if media
    // such as a YT video or Twitch Stream is playing on other devices.
    let mut player = player_finder.find_active()?;
    let mpris_metadata = player.get_metadata()?;

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

        let lrclib = LRCLib::new()?;

        let lyrics_finder = lyrics_finder!(&lrclib);

        let response = lyrics_finder.search(query.clone())?;

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

        let lrc = Lyrics::from_str(lyrics.unwrap())?;
        player = player_finder.find_active().unwrap();

        let start_timed_lyric = [(TimeTag::new(0), "".into())];

        let mut peekable_timed_lines = start_timed_lyric
            .iter()
            .chain(lrc.get_timed_lines())
            .peekable();

        while let Some((tag, line)) = peekable_timed_lines.next() {
            let next_timestamp = peekable_timed_lines.peek().map_or_else(
                || {
                    mpris_metadata
                        .length()
                        .map(|duration| duration.as_micros() as i64)
                        .unwrap_or_else(|| i64::MAX)
                },
                |(tag, _)| tag.get_timestamp(),
            );

            let time_range = tag.get_timestamp()..next_timestamp;

            let mut position = player.get_position().unwrap().as_millis() as i64;

            let mut flag = false;

            while !time_range.contains(&position) {
                position = player.get_position().unwrap().as_millis() as i64;

                if position > next_timestamp {
                    flag = true;
                    break;
                }
            }

            if !flag {
                println!("{} {}", tag, line);
            }
        }
    } else {
        return Err(anyhow!("Failed to parse lyrics"));
    }

    Ok(())
}
