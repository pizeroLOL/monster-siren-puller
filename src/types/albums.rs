use serde::{Deserialize, Serialize};
use crate::API;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlbumIndex {
    cid: String,
    name: String,
    coverUrl: String,
    artistes: Vec<String>,
}

impl AlbumIndex {
    pub fn get_url() -> String {
        format!("{API}albums/")
    }
    pub fn get_cid(&self) -> &str {
        &self.cid
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_cover_url(&self) -> &str {
        &self.coverUrl
    }
    pub fn get_artistes(&self) -> &Vec<String> {
        &self.artistes
    }
}
