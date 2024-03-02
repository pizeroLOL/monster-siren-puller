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
    pub fn get_url(cid: &str) -> String {
        format!("{API}song/{cid}")
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_source_url(&self) -> Option<&str> {
        self.sourceUrl.as_deref()
    }
    pub fn get_lyric_url(&self) -> Option<&str> {
        self.lyricUrl.as_deref()
    }
    pub fn get_mv_url(&self) -> Option<&str> {
        self.mvUrl.as_deref()
    }
    pub fn get_mv_cover_url(&self) -> Option<&str> {
        self.mvCoverUrl.as_deref()
    }
    pub fn get_artists(&self) -> &[String] {
        &self.artists
    }
}