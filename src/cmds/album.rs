use std::{error::Error, path::Path};

use monster_siren_puller::{
    download::download_album,
    types::{Album, SongIndex},
};

pub struct AlbumCmd;

impl AlbumCmd {
    fn arts_format(song: &SongIndex) -> String {
        let arts = song
            .get_artistes()
            .iter()
            .map(|arts| "\n\t\t".to_owned() + arts)
            .collect::<String>();
        format!(
            "\n\t名称: {}\n\tcid: {}\n\t艺术家: {arts}",
            song.get_name(),
            song.get_cid()
        )
    }
    fn about_song_fmt(song: &[SongIndex]) -> String {
        song.iter().map(Self::arts_format).collect()
    }

    pub async fn about(cid: usize) -> Result<(), Box<dyn Error>> {
        let cid = cid.to_string();
        let album = Album::get(&cid).await?;
        let album_name = album.get_name();
        let album_intro = album
            .get_intro()
            .lines()
            .map(|line| "\n\t".to_owned() + line)
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
        let album = Album::get(&cid).await?;
        let songs = album
            .get_songs()
            .iter()
            .map(|song| song.get_cid().to_owned() + "\t" + song.get_name() + "\n")
            .collect::<String>();
        println!("cid \t 名称\n{}", songs);
        Ok(())
    }

    pub async fn get(dir: &Path, cid: usize) -> Result<(), Box<dyn Error>> {
        let cid = cid.to_string();
        let album = Album::get(&cid).await?;
        let dir_name = album.get_name();
        download_album(&cid, dir, dir_name).await?;
        Ok(())
    }
}
