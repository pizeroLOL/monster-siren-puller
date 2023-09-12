use crate::API;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Song {
    cid: String,
    name: String,
    albumCid: String,
    sourceUrl: Option<String>,
    lyricUrl: Option<String>,
    mvUrl: Option<String>,
    mvCoverUrl: Option<String>,
    artists: Vec<String>,
}

impl Song {
    pub fn new_url(cid: &str) -> String {
        format!("{API}song/{cid}")
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_source_url(&self) -> &Option<String> {
        &self.sourceUrl
    }
    pub fn get_lyric_url(&self) -> &Option<String> {
        &self.lyricUrl
    }
    pub fn get_mv_url(&self) -> &Option<String> {
        &self.mvUrl
    }
    pub fn get_mv_cover_url(&self) -> &Option<String> {
        &self.mvCoverUrl
    }
    pub fn get_artists(&self) -> &Vec<String> {
        &self.artists
    }
}
