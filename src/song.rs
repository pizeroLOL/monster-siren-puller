use std::error::Error;

use serde::{Deserialize, Serialize};
use tokio::runtime::Builder;

use crate::{download::download, API};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    code: isize,
    msg: String,
    data: Song,
}

impl Request {
    pub fn get(cid: &str) -> Result<Self, Box<dyn Error>> {
        let runtime = Builder::new_multi_thread().enable_all().build()?;
        let t = runtime.block_on(async { get_song(cid).await })?;
        Ok(t)
    }
    pub fn to_song(&self) -> &Song {
        &self.data
    }
}

pub async fn get_song(cid: &str) -> Result<Request, Box<dyn Error>> {
    Ok(download(&(API.to_owned() + "song/" + cid))
        .await?
        .json::<Request>()
        .await?)
}

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
