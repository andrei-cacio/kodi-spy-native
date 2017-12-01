## Kodi Spy Native [WIP]

A complete rewrite of [kodi-spy (nodejs)](https://github.com/andrei-cacio/kodi-spy) written in Rust.

### Compile

```bash
$ git clone https://github.com/andrei-cacio/kodi-spy-native
$ cd kodi-spy-native
$ cargo build --release
```

### Usage

```bash
$ ./targer/release/kodi-spy-native {path_to_media_folder} {kodi_http_url}
```

### Features
[x] - CLI interface
[x] - Listen for file changes
[x] - Native notification for when a video was detected
[ ] - Call Kodi API to update the library
