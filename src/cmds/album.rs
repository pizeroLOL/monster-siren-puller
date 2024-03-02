use std::error::Error;

use monster_siren_puller::{
    download::{
        build_info::write_infos,
        config::DLConfig,
        downloading::{create_dirs, download_tasks},
        padding::{get_albums_tasks, get_song_indexes, get_songs_tasks},
    },
    types::{Album, Response, SongIndex},
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

    pub async fn about(cid: usize, config: &DLConfig) -> Result<(), reqwest::Error> {
        let cid = cid.to_string();
        let album = Response::<Album>::get(&Album::get_url(&cid), config).await?;
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

    pub async fn show(cid: usize, config: &DLConfig) -> Result<(), reqwest::Error> {
        let cid = cid.to_string();
        let album = Response::<Album>::get(&Album::get_url(&cid), config).await?;
        let songs = album.get_songs();
        println!("cid \t 名称");
        for song in songs {
            println!("{} \t {}", song.get_cid().to_owned(), song.get_name());
        }
        Ok(())
    }

    pub async fn get(cid: usize, config: &DLConfig) -> Result<(), Box<dyn Error>> {
        let cid = cid.to_string();
        let album = Response::<Album>::get(&Album::get_url(&cid), config).await?;
        let mut tasks = Vec::new();
        tasks.append(&mut get_albums_tasks(&[album.clone()]).to_vec());
        let song_indexes = get_song_indexes(&[album.clone()]);
        let songs_tasks = get_songs_tasks(song_indexes, config).await?;
        tasks.append(&mut songs_tasks.to_vec());
        create_dirs(&config.dir, &tasks)?;
        download_tasks(&tasks, config).await?;
        write_infos(&[album], &config.dir)?;
        Ok(())
    }
}
