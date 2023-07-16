use crate::{download::download, response::Response, song_index::SongIndex, API};
use serde::{Deserialize, Serialize};
use std::error::Error;

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
    pub async fn get(cid: &str) -> Result<Album, Box<dyn Error>> {
        let o = download(&(API.to_owned() + "album/" + cid + "/detail"))
            .await?
            .json::<Response<Album>>()
            .await?
            .data;
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
