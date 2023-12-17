use super::{config::DLConfig, REPLACE};
use crate::types::{Album, AlbumIndex, Response, Song};
use std::path::{Path, PathBuf};
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub struct DLTask {
    pub album: String,
    pub asset: String,
    pub url: String,
}

impl DLTask {
    pub fn new(album: String, asset: String, url: String) -> Self {
        Self { album, asset, url }
    }
    pub fn dir(&self, path: &Path) -> PathBuf {
        path.join(&self.album)
    }
    pub fn path(&self, path: &Path) -> PathBuf {
        self.dir(path).join(&self.asset)
    }
}

pub fn get_url_name(name: &str, url: &str) -> String {
    name.to_owned() + "." + url.rsplit('.').next().unwrap()
}

pub fn get_albums_tasks(albums: &[Album]) -> &'static [DLTask] {
    let mut tasks = Vec::new();
    for album in albums {
        let album_name = album.get_name().replace(REPLACE, "");
        let album_name = album_name.trim();
        tasks.push(DLTask::new(
            album_name.to_string(),
            get_url_name("cover", album.get_cover_url()),
            album.get_cover_url().to_string(),
        ));
        tasks.push(DLTask::new(
            album_name.to_string(),
            get_url_name("head", album.get_cover_de_url()),
            album.get_cover_de_url().to_string(),
        ));
    }
    tasks.leak()
}

pub fn get_song_indexes(albums: &[Album]) -> Vec<(String, String)> {
    #[inline]
    fn get_name_cid(album: &Album) -> Vec<(String, String)> {
        album
            .get_songs()
            .iter()
            .map(|x| {
                let name = album.get_name().replace(REPLACE, "").trim().to_string();
                (name, x.get_cid().to_string())
            })
            .collect::<Vec<_>>()
    }

    let song_indexes = albums.iter().flat_map(get_name_cid).collect::<Vec<_>>();
    song_indexes
}

fn get_song_tasks(song: Song, album_name: &str) -> &'static [DLTask] {
    let song_name = song.get_name().replace(REPLACE, "");
    let tasks = [
        song.get_source_url(),
        song.get_lyric_url(),
        song.get_mv_url(),
        song.get_mv_cover_url(),
    ]
    .into_iter()
    .flatten()
    .map(|x| x.to_string())
    .map(|x| DLTask::new(album_name.to_owned(), get_url_name(&song_name, &x), x))
    .collect::<Vec<_>>();
    tasks.leak()
}

pub async fn get_albums(
    album_indexes: &[AlbumIndex],
    config: &DLConfig,
) -> Result<Vec<Album>, reqwest::Error> {
    let mut albums = Vec::new();
    for (index, album_index) in album_indexes.iter().enumerate() {
        if index % 20 == 0 {
            sleep(config.retry_time).await;
        }
        let url = Album::get_url(album_index.get_cid());
        let album = Response::<Album>::get(&url, config).await?;
        albums.push(album)
    }
    Ok(albums)
}

pub async fn get_songs_tasks(
    song_indexes: Vec<(String, String)>,
    config: &DLConfig,
) -> Result<&[DLTask], reqwest::Error> {
    let mut tmp = Vec::new();
    for (index, (album_name, cid)) in song_indexes.iter().enumerate() {
        let album_name = album_name.replace(REPLACE, "");
        let album_name = album_name.trim();
        if index % 20 == 0 {
            sleep(config.retry_time).await;
        }
        let url = Song::get_url(cid);
        let song = Response::<Song>::get(&url, config).await?;
        tmp.append(&mut get_song_tasks(song, album_name).to_vec())
    }
    Ok(tmp.leak())
}

pub async fn from_album_indexes(
    album_indexes: &[AlbumIndex],
    config: &DLConfig,
) -> Result<(&'static [DLTask], &'static [Album]), reqwest::Error> {
    let mut tasks = Vec::new();
    let albums = get_albums(album_indexes, config).await?;
    tasks.append(&mut get_albums_tasks(&albums).to_vec());
    let song_indexes = get_song_indexes(&albums);
    let songs_tasks = get_songs_tasks(song_indexes, config).await?;
    tasks.append(&mut songs_tasks.to_vec());
    Ok((tasks.leak(), albums.leak()))
}
