use std::time::Duration;
use clap::{Parser, ValueEnum};
use futures::StreamExt;
use youtube_collector::printers::Printers;
use youtube_collector::yt_client::{YTChannelID, YTClient, YTKey};

#[derive(Copy, Clone, ValueEnum, Debug)]
pub enum Format { String, Json, CSV }

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(long, value_name = "YouTube Data API Key")]                   yt_key: YTKey,
    #[arg(long, value_name = "YouTube Channel ID")]                     yt_channel: YTChannelID,
    #[arg(short, long, value_enum, default_value_t = Format::String)]   format: Format,
}

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();
    let client = YTClient::from_agent(
        args.yt_key,
        ureq::AgentBuilder::new().timeout_read(Duration::from_secs(1)).build(),
    );

    let video_ids = client
        .search(args.yt_channel)
        .expect("Failed fetching YouTube video IDs.");

    let video_requests = video_ids
        .into_iter()
        .map(|video_id| async {
            client.videos(video_id)
                .expect("Failed fetching YouTube videos.")
                .first()
                .expect("Collection of videos is empy.")
                .clone()
        });

    let stream = futures::stream::iter(video_requests)
        .buffer_unordered(4);

    let video_items = stream.collect::<Vec<_>>().await;

    match args.format {
        Format::CSV => Printers::print_csv(&video_items),
        Format::Json => Printers::print_json(&video_items),
        _ => Printers::print_string(&video_items),
    }
}
