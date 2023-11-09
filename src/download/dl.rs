use std::error::Error;

use monster_siren_puller::types::{Album, AlbumIndex, Response, Song};
use reqwest::Client;

use super::{task::Task, REPLACE};

pub fn gen_client(task: &Task) -> Result<Client, Box<dyn Error>> {
    let client = Client::builder()
        .user_agent(&task.ua)
        .timeout(task.timeout)
        .build()?;
    Ok(client)
}

pub async fn download(url:&str) {

}

pub async fn get_song(task: &Task, url: &str) -> Result<(), Box<dyn Error>> {
    let client = gen_client(task)?;
    let song = gen_client(task)?
        .get(url)
        .send()
        .await?
        .json::<Response<Song>>()
        .await?
        .data;

    let tasks = vec![
        song.get_source_url(),
        song.get_mv_url(),
        song.get_mv_cover_url(),
        song.get_lyric_url(),
    ]
    .into_iter()
    .flatten()
    .map(|x| )
    .collect::<Vec<_>>();
    Ok(())
}

pub async fn get_album_tasks(task: &Task, cid: &str, name: &str) -> Result<(), Box<dyn Error>> {
    let name = name.trim().replace(REPLACE, "");
    let dir = task.path.join(name);
    let dir = dir.as_path();
    tokio::fs::create_dir_all(dir).await?;
    let album = gen_client(task)?
        .get(Album::new_url(cid))
        .send()
        .await?
        .json::<Response<Album>>()
        .await?
        .data;
    let tasks = vec![
        ("head", album.get_cover_url()),
        ("wide_head", album.get_cover_de_url()),
    ];
    let song_tasks = album
        .get_songs()
        .iter()
        .map(|song| Song::new_url(song.get_cid()));

    Ok(())
}

pub async fn donwload_all(task: &Task) -> Result<(), Box<dyn Error>> {
    let t = gen_client(task)?
        .get(AlbumIndex::new_url())
        .send()
        .await?
        .json::<Response<Vec<AlbumIndex>>>()
        .await?
        .data
        .iter()
        .map(|x| (x.get_cid(), x.get_name()));
    // .map(|x|);
    Ok(())
}
