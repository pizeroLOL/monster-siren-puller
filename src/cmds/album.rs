use std::{env::Args, error::Error, path::Path};

use monster_siren_puller::{
    album_detail::{get_album, SongIndex},
    download::download_album,
};

pub struct Album;

impl Album {
    pub async fn main(env: Args) -> Result<(), Box<dyn Error>> {
        let mut env = env;
        let subcmd;
        if let Some(cmd) = env.next() {
            subcmd = cmd;
        } else {
            Self::help();
            return Err("No subcmd".into());
        };
        let subcmd = subcmd.as_str();
        match subcmd {
            "help" => Self::help(),
            "about" => Self::about(env).await?,
            "show" => Self::show(env).await?,
            "get" => Self::get(env).await?,
            _ => Self::help(),
        };
        Ok(())
    }

    fn about_song_fmt(song: &Vec<SongIndex>) -> String {
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

    async fn about(env: Args) -> Result<(), Box<dyn Error>> {
        let cid;
        let mut env = env;
        if let Some(cmd) = env.next() {
            cid = cmd;
        } else {
            Self::help();
            return Err("No cid".into());
        };
        let album = get_album(&cid).await?;
        let album = album.to_album();
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

    async fn show(env: Args) -> Result<(), Box<dyn Error>> {
        let mut env = env;
        let cid;
        if let Some(cmd) = env.next() {
            cid = cmd;
        } else {
            Self::help();
            return Err("No cid".into());
        };

        let album = get_album(&cid).await?;
        let songs = album
            .to_album()
            .get_songs()
            .iter()
            .map(|song| format!("{} \t {} \n", song.get_cid(), song.get_name()))
            .collect::<String>();
        println!("cid \t 名称\n{}", songs);
        Ok(())
    }

    async fn get(cid: Args) -> Result<(), Box<dyn Error>> {
        let mut cid = cid;
        let tmp;
        if let Some(cmd) = cid.next() {
            tmp = cmd;
        } else {
            Self::help();
            return Err("No cid".into());
        };
        let cid = tmp;

        let album = get_album(&cid).await?;
        let dir_name = album.to_album().get_name();

        download_album(&cid, Path::new("./siren/"), dir_name).await?;

        Ok(())
    }

    fn help() {
        println!(
            "
moster-siren-puller album [about|help]
moster-siren-puller album [list|get] <cid>

help \t获取该消息
about\t显示专辑相关信息
show \t获取专辑下的歌曲列表
get  \t下载专辑（TODO）
        "
        )
    }
}
