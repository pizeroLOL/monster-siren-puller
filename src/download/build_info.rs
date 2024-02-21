use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use crate::types::Album;

use super::REPLACE;

pub fn format_song_artistes(name: &str, artistes: &[String], len: usize) -> String {
    let artistes = artistes
        .iter()
        .enumerate()
        .map(|(index, artist_name)| match index + 1 >= len {
            true => artist_name.to_string(),
            false => format!("{artist_name}、"),
        })
        .collect::<String>();
    format!("歌曲：{name}\t作者：{artistes}\n")
}

pub fn format_info(data: &Album) -> String {
    let songs = data.get_songs();
    let t_max = songs.len();
    let t = songs
        .iter()
        .map(|x| format_song_artistes(x.get_name(), x.get_artistes(), t_max))
        .collect::<String>();
    format!(
        "专辑名：{}\n简介：{}\n{}",
        data.get_name(),
        data.get_intro(),
        t
    )
}

pub fn write_infos(albums: &[Album], path: &Path) -> io::Result<()> {
    for album in albums {
        let album_name = album.get_name().replace(REPLACE, "");
        let album_name = album_name.trim();
        let path = path.join(album_name).join("info.txt");
        let mut file = File::create(path)?;
        file.write_all(format_info(album).as_bytes())?;
    }
    Ok(())
}
