use std::{error::Error, path::Path};

use monster_siren_puller::{
    download_interface::download_album,
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

    pub async fn about(cid: usize) -> Result<(), Box<dyn Error>> {
        let cid = cid.to_string();
        let album = Album::get(&cid).await?;
        let album_intro = album
            .get_intro()
            .lines()
            .map(|line| "\n\t".to_owned() + line)
            .collect::<String>();
        let album_songs = album
            .get_songs()
            .iter()
            .map(Self::arts_format)
            .collect::<String>();
        println!(
            "名称: {}\n专辑说明: {album_intro}\n封面链接: {}\n封面展示链接: {}\n歌曲: {album_songs}",
            album.get_name(),
            album.get_cover_url(),
            album.get_cover_de_url()
        );
        Ok(())
    }

    pub async fn show(cid: usize) -> Result<(), Box<dyn Error>> {
        let cid = cid.to_string();
        let album = Album::get(&cid).await?;
        let songs = album.get_songs();
        println!("cid \t 名称");
        for song in songs {
            println!("{} \t {}", song.get_cid().to_owned(), song.get_name());
        }
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
