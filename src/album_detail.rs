use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::runtime::Builder;

use crate::{download::download, API};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    code: isize,
    msg: String,
    pub data: Album,
}

impl Request {
    pub fn from(cid: &str) -> Result<Self, Box<dyn Error>> {
        let runtime = Builder::new_multi_thread().enable_all().build()?;
        let t = runtime.block_on(async { get_album(cid).await })?;
        Ok(t)
    }
    pub fn to_album(self) -> Album {
        self.data
    }
}

pub async fn get_album(cid: &str) -> Result<Request, Box<dyn Error>> {
    Ok(download(&(API.to_owned() + "album/" + cid + "/detail"))
        .await?
        .json::<Request>()
        .await?)
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Album {
    cid: String,
    pub name: String,
    pub intro: String,
    pub coverUrl: String,
    pub coverDeUrl: String,
    pub songs: Vec<SongIndex>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SongIndex {
    pub cid: String,
    pub name: String,
    pub artistes: Vec<String>,
}
