use std::error::Error;

use crate::{download::download, API, USER_AGENT};
use serde::{Deserialize, Serialize};
use tokio::runtime::Builder;

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    code: isize,
    msg: String,
    pub data: Vec<Index>,
}

impl Request {
    pub fn new(code: isize, msg: String, data: Vec<Index>) -> Self {
        Self { code, msg, data }
    }
    pub fn get() -> Result<Self, Box<dyn Error>> {
        let runtime = Builder::new_multi_thread().enable_all().build()?;
        Ok(runtime.block_on(async { get_albums_index().await })?)
    }
    pub fn to_index_list(self) -> Vec<Index> {
        self.data
    }
}

pub async fn get_albums_index() -> Result<Request, Box<dyn Error>> {
    Ok(download(&(API.to_owned() + "albums/"))
        .await?
        .json::<Request>()
        .await?)
}

#[warn(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Index {
    pub cid: String,
    pub name: String,
    coverUrl: String,
    artistes: Vec<String>,
}
