use super::song_index::SongIndex;
use crate::API;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
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