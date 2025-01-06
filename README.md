# Paroles

A rust-based cli tool and service to fetch synced lyrics and synchronize them with the active MPRIS Player.

## Todo List

- [x] Get active mpris player metadata
- [x] Search lyrics
    - [x] track name
    - [x] artist name
    - [x] album name
    - [x] duration
    - [ ] matching words
- [ ] Multiple sources
    - [x] LRCLib
- [x] Parse lyrics
- [x] Sync lyrics to active mpris player 
    - [ ] Interrupt on player/track change/stop/pause
- [ ] Ignore when no synced lyrics available
- [ ] Cache lyrics
- [ ] Write current line to a UnixSocket

_Note:_ Paroles is a work in progress and may change during its development.
