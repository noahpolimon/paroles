# Paroles

A cli tool and service to fetch synced lyrics and synchronize them with active playing media written in Rust.

## Build Instructions

### Note that, Paroles, as of now, is only supported on Linux and BSD systems.

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

_Note:_ At this point in development, paroles is not ready for use, so just use the `cargo run` command for now.

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
- [ ] Windows Support

_Note:_ Paroles is a work in progress and may change during its development.

## License

```
MIT License

Copyright (c) 2025 noahpolimon

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
