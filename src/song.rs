use std::error::Error;

use serde::{Deserialize, Serialize};
use tokio::runtime::Builder;

use crate::{download::download, API};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    code: isize,
    msg: String,
    pub data: Song,
}

impl Request {
    pub fn get(cid: &str) -> Result<Self, Box<dyn Error>> {
        let runtime = Builder::new_multi_thread().enable_all().build()?;
        Ok(runtime.block_on(async { get_song(cid).await })?)
    }
    pub fn to_song(self) -> Song {
        self.data
    }
}

pub async fn get_song(cid: &str) -> Result<Request, Box<dyn Error>> {
    Ok(download(&(API.to_owned() + "song/" + cid))
        .await?
        .json::<Request>()
        .await?)
}

#[warn(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Song {
    cid: String,
    pub name: String,
    albumCid: String,
    pub sourceUrl: String,
    pub lyricUrl: Option<String>,
    pub mvUrl: Option<String>,
    pub mvCoverUrl: Option<String>,
    pub artists: Vec<String>,
}
