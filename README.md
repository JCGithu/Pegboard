# Pegboard

A collection of command-line tools for simple tasks. Written in Rust.

All tools only apply to the local directory and sub-directories.

To build the tools individually run `cargo build --bin <tool> --release`.

### `songsort`

Generates iTunes-style folder structure for music files using metadata. Moves files to the correct folder.

Artist -> album -> tracks.

Requires ffprobe.

### `flac2mp3`

Converts flac files to mp3, optionally deleting the originals. Requires ffmpeg.

### `unbox`

Unzips every compressed file in the current directory.
Add `--logging` to see what's going on.

Requires 7z.

# To Do

Add the Inquire dependency for an easy CLI multi-select. Flac2mp3 will be changed to accept multiple file types.
[mikaelmello/inquire: A Rust library for building interactive prompts](https://github.com/mikaelmello/inquire)
