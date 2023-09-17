use super::{Task, TaskError, REPLACE};

impl<'a> Task<'a> {
    pub async fn download_all(&self) -> Result<(), TaskError> {
        let tasks = self.get_cids().await?;
        tokio::fs::create_dir_all(self.path.as_path())
            .await
            .map_err(TaskError::CreateDir)?;
        for (cid, name) in tasks {
            self.download_album(&cid, &name).await?
        }

        Ok(())
    }

    pub async fn download_album(&self, cid: &str, name: &str) -> Result<(), TaskError> {
        let name = name.trim().replace(REPLACE, "");
        let dir = self.path.join(name);
        let dir = dir.as_path();
        tokio::fs::create_dir_all(dir)
            .await
            .map_err(TaskError::CreateDir)?;
        let album_task = self.set_path(dir);
        let album = self.get_album(cid).await?;
        let mut img_task = Vec::new();
        for (name, url) in [
            ("head", album.get_cover_url()),
            ("wide_head", album.get_cover_de_url()),
        ] {
            img_task.push(self.get_write_url(url, name));
        }
        let re = futures::future::join_all(img_task)
            .await
            .into_iter()
            .filter_map(|e| e.err())
            .collect::<Vec<_>>();
        if !re.is_empty() {
            return Err(TaskError::DownloadImage(re));
        }

        let songs = album
            .get_songs()
            .iter()
            .map(|song| album_task.download_album_song(song.get_cid()))
            .collect::<Vec<_>>();
        let tasks = futures::future::join_all(songs)
            .await
            .into_iter()
            .filter_map(|e| e.err())
            .collect::<Vec<_>>();
        if !tasks.is_empty() {
            return Err(TaskError::DownloadSongs(tasks));
        }

        Ok(())
    }

    pub async fn download_album_song(&self, cid: &str) -> Result<(), TaskError> {
        let song = self.get_song(cid).await?;
        let song = [
            song.get_lyric_url(),
            song.get_source_url(),
            song.get_mv_cover_url(),
            song.get_mv_url(),
        ]
        .into_iter()
        .flatten()
        .map(|x| self.get_write_url(x, song.get_name()))
        .collect::<Vec<_>>();
        let re = futures::future::join_all(song)
            .await
            .into_iter()
            .filter_map(|e| e.err())
            .collect::<Vec<_>>();

        if !re.is_empty() {
            return Err(TaskError::DownloadSongAssest(re));
        }
        Ok(())
    }
}
