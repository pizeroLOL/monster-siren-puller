use crate::{download::download, request::Response, API};
use serde::{Deserialize, Serialize};
use std::error::Error;

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
    pub async fn get(cid: &str) -> Result<Song, Box<dyn Error>> {
        let o = download(&(API.to_owned() + "song/" + cid))
            .await?
            .json::<Response<Song>>()
            .await?
            .data;
        Ok(o)
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
