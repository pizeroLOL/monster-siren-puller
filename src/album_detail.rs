use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::runtime::Builder;

use crate::{download::download, API};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    code: isize,
    msg: String,
    data: Album,
}

impl Request {
    pub fn from(cid: &str) -> Result<Self, Box<dyn Error>> {
        let runtime = Builder::new_multi_thread().enable_all().build()?;
        let t = runtime.block_on(async { get_album(cid).await })?;
        Ok(t)
    }
    pub fn to_album(&self) -> &Album {
        &self.data
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
    name: String,
    intro: String,
    coverUrl: String,
    coverDeUrl: String,
    songs: Vec<SongIndex>,
}

impl Album {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_intro(&self) -> &str {
        &self.intro
    }
    pub fn get_cover_url(&self) -> &str {
        &self.coverUrl
    }
    pub fn get_cover_de_url(&self) -> &str {
        &self.coverDeUrl
    }
    pub fn get_songs(&self) -> &Vec<SongIndex> {
        &self.songs
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SongIndex {
    cid: String,
    name: String,
    artistes: Vec<String>,
}

impl SongIndex {
    pub fn get_cid(&self) -> &str {
        &self.cid
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_artistes(&self) -> &Vec<String> {
        &self.artistes
    }
}
