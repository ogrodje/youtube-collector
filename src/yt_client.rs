use crate::yt_error::YTError;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::str::FromStr;
use ureq::serde::de::DeserializeOwned;
use ureq::{serde_json, Agent, Response};

pub type YTKey = String;
pub type YTChannelID = String;
pub type VideoID = String;

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchItems {
    #[serde(rename = "items")]
    items: Vec<Item>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VideoItems {
    #[serde(rename = "items")]
    items: Vec<VideoItem>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ItemKind {
    kind: String,
    #[serde(rename = "videoId")]
    video_id: VideoID,
}

fn number_from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: serde::de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Statistics {
    #[serde(rename = "viewCount", deserialize_with = "number_from_string")]
    pub view_count: u64,
    #[serde(rename = "likeCount", deserialize_with = "number_from_string")]
    pub like_count: u64,
    #[serde(rename = "favoriteCount", deserialize_with = "number_from_string")]
    pub favorite_count: u64,
    #[serde(rename = "commentCount", deserialize_with = "number_from_string")]
    pub comment_count: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VideoItem {
    kind: String,
    etag: String,
    pub id: VideoID,
    pub statistics: Statistics,
    pub snippet: Snippet,
}

/*
impl Copy for VideoItem {}

impl Clone for VideoItem {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for String {}
impl Clone for String {
    fn clone(&self) -> Self {
        *self
    }
}
*/

#[derive(Deserialize, Serialize, Debug)]
struct Item {
    kind: String,
    etag: String,
    id: ItemKind,
    snippet: Option<Snippet>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Snippet {
    pub title: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct ChannelId {
    #[serde(rename = "channelId")]
    channel_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct VideoId {
    #[serde(rename = "videoId")]
    video_id: VideoID,
}

pub struct YTClient {
    yt_key: YTKey,
    agent: Agent,
}

impl YTClient {
    pub fn from_agent(yt_key: YTKey, agent: Agent) -> Self {
        YTClient { agent, yt_key }
    }

    fn parse_json_to<T>(response: Response) -> Result<T, YTError>
    where
        T: DeserializeOwned,
    {
        response
            .into_json()
            .map_err(|e| YTError::JSONParsingError(e.to_string()))
            .and_then(|json| {
                serde_json::from_value::<T>(json)
                    .map_err(|e| YTError::JSONValueConversionError(e.to_string()))
            })
    }

    fn with_query<'a>(&'a self, pairs: Vec<(&'a str, &'a str)>) -> Vec<(&str, &str)> {
        [vec![("key", self.yt_key.as_str())], pairs].concat()
    }

    fn parse_to_search_items(response: Response) -> Result<Vec<VideoID>, YTError> {
        Self::parse_json_to::<SearchItems>(response).and_then(|search_items| {
            Ok(search_items
                .items
                .iter()
                .map(|i| i.id.video_id.to_string())
                .collect())
        })
    }

    fn parse_to_video_items(response: Response) -> Result<Vec<VideoItem>, YTError> {
        Self::parse_json_to::<VideoItems>(response).and_then(|search_items| Ok(search_items.items))
    }

    pub fn search(&self, channel_id: YTChannelID) -> Result<Vec<VideoID>, YTError> {
        self.agent
            .get("https://www.googleapis.com/youtube/v3/search")
            .query_pairs(self.with_query(vec![
                ("channelId", channel_id.as_str()),
                ("part", "id"),
                ("type", "video"),
                ("maxResults", "50"),
                ("order", "date"),
            ]))
            .call()
            .map_err(YTError::to_yt_error)
            .and_then(Self::parse_to_search_items)
    }

    pub fn videos(&self, video_id: VideoID) -> Result<Vec<VideoItem>, YTError> {
        self.agent
            .get("https://www.googleapis.com/youtube/v3/videos")
            .query_pairs(self.with_query(vec![
                ("id", video_id.as_str()),
                ("part", "statistics,contentDetails,snippet"),
            ]))
            .call()
            .map_err(YTError::to_yt_error)
            .and_then(Self::parse_to_video_items)
    }
}
