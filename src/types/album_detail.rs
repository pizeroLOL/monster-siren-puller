use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::{download::download, API};

use super::{response::Response, song_index::SongIndex};

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
    pub fn get_url(cid: &str) -> String {
        format!("{API}album/{cid}/detail")
    }
    pub async fn get(cid: &str) -> Result<Album, Box<dyn Error>> {
        let url = Self::get_url(cid);
        let o = download(&url).await?.json::<Response<Album>>().await?.data;
        Ok(o)
    }
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
