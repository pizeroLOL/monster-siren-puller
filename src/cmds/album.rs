use std::{error::Error, path::Path};

use monster_siren_puller::types::{Album, SongIndex};

use crate::download_task::Task;

pub struct AlbumCmd;

impl AlbumCmd {
    fn about_song_fmt(song: &[SongIndex]) -> String {
        song.iter()
            .map(|songs| {
                let name = songs.get_name();
                let cid = songs.get_cid();
                let artistes = songs
                    .get_artistes()
                    .iter()
                    .map(|arts| format!("\n\t\t{arts}"))
                    .collect::<String>();
                format!("\n\t名称: {name}\n\tcid: {cid}\n\t艺术家: {artistes}",)
            })
            .collect::<String>()
    }

    pub async fn about(cid: usize) -> Result<(), Box<dyn Error>> {
        let cid = cid.to_string();
        let album = Task::new(Path::new("."))
            .get_album(&cid.to_string())
            .await
            .map_err(|e| format!("{e:?}"))?;
        let album_name = album.get_name();
        let album_intro = album
            .get_intro()
            .lines()
            .map(|line| format!("\n\t{line}"))
            .collect::<String>();
        let album_cover_url = album.get_cover_url();
        let album_cover_de_url = album.get_cover_de_url();
        let album_songs = Self::about_song_fmt(album.get_songs());
        let output = format!(
            "
名称: {album_name}
专辑说明: {album_intro}
封面链接: {album_cover_url}
封面展示链接: {album_cover_de_url}
歌曲: {album_songs}"
        );
        println!("{}", output);
        Ok(())
    }

    pub async fn show(cid: usize) -> Result<(), Box<dyn Error>> {
        let cid = cid.to_string();
        // let album = Album::get(&cid).await?;
        let album = Task::new(Path::new("."))
            .get_album(&cid.to_string())
            .await
            .map_err(|e| format!("{e:?}"))?;
        let songs = album
            .get_songs()
            .iter()
            .map(|song| format!("{} \t {} \n", song.get_cid(), song.get_name()))
            .collect::<String>();
        println!("cid \t 名称\n{}", songs);
        Ok(())
    }

    pub async fn get(dir: &Path, cid: usize) -> Result<(), Box<dyn Error>> {
        let cid = cid.to_string();
        let album = Task::new(Path::new("."))
            .get_album(&cid)
            .await
            .map_err(|e| format!("{e:?}"))?;
        let dir_name = album.get_name();
        // download_album(&cid, dir, dir_name).await?;
        // Ok(())
        todo!()
    }
}
