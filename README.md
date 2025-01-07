# Paroles

A rust-based cli tool and service to fetch synced lyrics and synchronize them with the active MPRIS Player.

## Build Instructions

To build it, you need to have rustup installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
```

Then, clone the repository and run it:

```bash
git clone https://github.com/noahpolimon/paroles.git
cd paroles
cargo run
```

_Note:_ At this point in development, paroles is not ready for use, so just use the `cargo run` for now.

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
- [ ] Ignore when no synced lyrics available (subject to removal)
- [ ] Cache lyrics
- [ ] Write current line to a UnixSocket

_Note:_ Paroles is a work in progress and may change during its development.
