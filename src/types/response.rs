use serde::{Deserialize, Serialize};

use crate::download::{config::DLConfig, downloading::download};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    pub code: isize,
    pub msg: String,
    pub data: T,
}

impl<T> Response<T>
where
    T: for<'a> Deserialize<'a>,
    Self: Sized,
{
    pub async fn get(url: &str, config: &DLConfig) -> Result<T, reqwest::Error> {
        let o = download(url, config)
            .await?
            .json::<Response<T>>()
            .await?
            .data;
        Ok(o)
    }
}
