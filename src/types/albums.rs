use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::{download::download, API};

use super::response::Response;

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
    pub async fn get() -> Result<Vec<AlbumIndex>, Box<dyn Error>> {
        let url = Self::get_url();
        let o = download(&url)
            .await?
            .json::<Response<Vec<AlbumIndex>>>()
            .await?
            .data;
        Ok(o)
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
