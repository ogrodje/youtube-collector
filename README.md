# youtube-collector

[youtube-collector] is a simple CLI application used for collection of YouTube channel and video statistics.

## Usage

1. [Enable YouTube Data APIs](https://console.cloud.google.com/apis/api/youtube.googleapis.com) on your Google Cloud
   project.
2. Get your API key in the Google Cloud Console.
3. Get your YouTube Channel ID in the "advanced settings" of your YouTube Channel.
4. Use the `cli` app:

    ```bash
    A library to extracttion of statistics from YouTube channels and videos.
    
    Usage: cli [OPTIONS] --yt-key <YouTube Data API Key> --yt-channel <YouTube Channel ID>
    
    Options:
          --yt-key <YouTube Data API Key>
          --yt-channel <YouTube Channel ID>
      -f, --format <FORMAT>                  [default: string] [possible values: string, json, csv]
      -h, --help                             Print help
      -V, --version                          Print version
    ```

## Development

For **development** build and running use:

```bash
cargo run -- --yt-key $YT_KEY --yt-channel $YT_CHANNEL_ID
```

To build **production** release use the cargo and build:

```bash
cargo build --release && \
  ./target/release/cli --yt-key $YT_KEY --yt-channel $YT_CHANNEL_ID --format json
```

## History

This CLI Rust tool was build for the collection of metrics for
the [Ogrodje Podcasts](https://podcasts.apple.com/si/podcast/ogrodje/id1623611047).
If you are looking for other similar tools, make sure you also check
the [anchor-collector](https://github.com/otobrglez/anchor-collector).

### Social && <3

[![GitHub stars](https://img.shields.io/github/stars/otobrglez/youtube-collector.svg?style=social&label=Star)](https://github.com/otobrglez/youtube-collector)
[![GitHub watchers](https://img.shields.io/github/watchers/otobrglez/youtube-collector.svg?style=social&label=Watch)](https://github.com/otobrglez/youtube-collector)
[![GitHub followers](https://img.shields.io/github/followers/otobrglez.svg?style=social&label=Follow)](https://github.com/otobrglez/youtube-collector)  
[![Twitter Follow](https://img.shields.io/twitter/follow/otobrglez.svg?style=social)](https://twitter.com/otobrglez)

[youtube-collector]: https://github.com/otobrglez/youtube-collector
