use std::error::Error;

use crate::{download::download, API};
use serde::{Deserialize, Serialize};
use tokio::runtime::Builder;

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    code: isize,
    msg: String,
    data: Vec<Index>,
}

impl Request {
    pub fn new(code: isize, msg: String, data: Vec<Index>) -> Self {
        Self { code, msg, data }
    }
    pub fn get() -> Result<Self, Box<dyn Error>> {
        let runtime = Builder::new_multi_thread().enable_all().build()?;
        let t = runtime.block_on(async { get_albums_index().await })?;
        Ok(t)
    }
    pub fn to_index_list(&self) -> &Vec<Index> {
        &self.data
    }
    pub fn get_data(&self) -> &Vec<Index> {
        &self.data
    }
}

pub async fn get_albums_index() -> Result<Request, Box<dyn Error>> {
    Ok(download(&(API.to_owned() + "albums/"))
        .await?
        .json::<Request>()
        .await?)
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Index {
    cid: String,
    name: String,
    coverUrl: String,
    artistes: Vec<String>,
}

impl Index {
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
