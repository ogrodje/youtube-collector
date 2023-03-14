use serde::{Deserialize, Serialize};
use ureq::serde_json;

use crate::yt_client::VideoItem;

pub struct Printers {}

#[derive(Serialize, Deserialize, Clone)]
struct Videos {
    videos: Vec<VideoItem>,
}

impl Printers {
    pub fn print_json(items: &Vec<VideoItem>) -> () {
        let json = serde_json::to_string(&Videos {
            videos: items.clone(),
        })
        .expect("Problem with serialization to JSON.");
        println!("{}", json);
    }

    pub fn print_csv(items: &Vec<VideoItem>) -> () {
        for item in items.iter() {
            println!(
                "{:?},{:?},{:?},{:?},{:?}",
                item.snippet.title,
                item.statistics.view_count,
                item.statistics.like_count,
                item.statistics.favorite_count,
                item.statistics.comment_count,
            );
        }
    }

    pub fn print_string(items: &Vec<VideoItem>) -> () {
        for item in items.iter() {
            println!("{} {}", item.snippet.title, item.statistics.view_count);
        }
    }
}
