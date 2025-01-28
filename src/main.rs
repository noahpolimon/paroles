// MIT License

// Copyright (c) 2025 noahpolimon

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

mod errors;
mod events;
mod playback;
mod player;
mod providers;
mod song;
mod utils;

use anyhow::{anyhow, Result};
use lrc::{Lyrics, TimeTag};
use providers::{lyrics_finder, LRCLib, Musixmatch, ProviderTrait, Response};
use song::SongInfo;

fn main() -> Result<()> {
    let player_finder = mpris::PlayerFinder::new()?;
    // FIXME: find other mpris player with status Playing if lyrics for this one is not found.
    //
    // Reason: applications such as KDEConnect also uses MPRIS, so it might default to that if media
    // such as a YT video or Twitch Stream is playing on other devices.
    let player = player_finder.find_active()?;

    let mpris_metadata = player.get_metadata()?;

    let song_info = SongInfo::try_from(&mpris_metadata)?;

    println!(
        "|> Now Playing: {}\n",
        song_info.to_full_title(Default::default())
    );

    let lrclib = LRCLib::new()?;
    let musixmatch = Musixmatch::new()?;

    let lyrics_finder = lyrics_finder!(&lrclib, &musixmatch);

    let response = lyrics_finder.find(&song_info)?;

    let mut index = 0;

    for lyrics in &response {
        if lyrics.synced_lyrics.is_some() {
            break;
        }

        index += 1;
    }

    let lyrics = if index == response.len() {
        return Err(anyhow!("Lyrics not found"));
    } else {
        response.get(index).unwrap().synced_lyrics.as_ref()
    };

    let mut lrc = Lyrics::from_str(lyrics.unwrap())?;
    lrc.add_timed_line(TimeTag::new(0), "")?;

    let mut peekable_timed_lines = lrc.get_timed_lines().iter().peekable();

    let millis_compensation = -500; // workaround for late lyrics

    'outer: while let Some((tag, line)) = peekable_timed_lines.next() {
        let current_timestamp = tag.get_timestamp() + millis_compensation;

        if current_timestamp == millis_compensation {
            continue;
        }

        let mut position = player.get_position().unwrap().as_millis() as i64;

        let next_timestamp = peekable_timed_lines.peek().map_or_else(
            || {
                mpris_metadata
                    .length()
                    .map(|duration| duration.as_millis() as i64)
                    .unwrap_or_else(|| i64::MAX)
            },
            |(tag, _)| tag.get_timestamp(),
        );

        let time_range = current_timestamp..next_timestamp;

        while !time_range.contains(&position) {
            if position > next_timestamp {
                continue 'outer;
            }
            position = player.get_position().unwrap().as_millis() as i64;
        }

        println!("{} {}", tag, line);
    }

    Ok(())
}
