use monster_siren_puller::types::{Album, AlbumIndex, Response, Song};

use super::{Task, TaskError};

impl<'a> Task<'a> {
    pub fn gen_client(&self) -> Result<reqwest::Client, TaskError> {
        reqwest::Client::builder()
            .user_agent(self.ua)
            .timeout(self.timeout)
            .build()
            .map_err( TaskError::GenClient)
    }

    async fn get_json<T>(&self, url: &str) -> Result<T, TaskError>
    where
        T: for<'de> serde::de::Deserialize<'de>,
    {
        let rsp = self
            .gen_client()?
            .get(url)
            .send()
            .await
            .map_err( TaskError::SendData)?;
        let data = rsp
            .json::<T>()
            .await
            .map_err(TaskError::Deserialize)?;
        Ok(data)
    }

    pub async fn get_cids(&self) -> Result<Vec<(String, String)>, TaskError> {
        let task = self
            .get_json::<Response<Vec<AlbumIndex>>>(&AlbumIndex::new_url())
            .await?
            .data
            .iter()
            .map(|x| (x.get_cid().to_string(), x.get_name().to_string()))
            .collect();
        Ok(task)
    }

    pub async fn get_album(&self, cid: &str) -> Result<Album, TaskError> {
        let album: Album = self
            .get_json::<Response<Album>>(&Album::new_url(cid))
            .await?
            .data;
        Ok(album)
    }

    pub async fn get_song(&self, cid: &str) -> Result<Song, TaskError> {
        let song = self
            .get_json::<Response<Song>>(&Song::new_url(cid))
            .await?
            .data;
        Ok(song)
    }

    pub async fn get_byte(&self, url: &str) -> Result<bytes::Bytes, TaskError> {
        let bytes = self
            .gen_client()?
            .get(url)
            .send()
            .await
            .map_err(TaskError::SendData)?
            .bytes()
            .await
            .map_err( TaskError::Deserialize)?;
        Ok(bytes)
    }
}
