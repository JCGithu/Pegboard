# Pegboard

A collection of command-line tools for simple tasks. Written in Rust.

All tools only apply to the local directory and sub-directories.

### `songsort`

Generates iTunes-style folder structure for music files using metadata. Moves files to the correct folder.

Artist -> album -> tracks.

Requires ffprobe.

### `flac2mp3`

Converts flac files to mp3, optionally deleting the originals.
Requires ffmpeg.

### `unbox`

Unzips every compressed file in the current directory.
Add `--logging` to see what's going on.

Requires 7z.
