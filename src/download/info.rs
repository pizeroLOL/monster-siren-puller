use crate::types::Album;

pub fn format_song_artistes(name: &str, artistes: &[String], len: usize) -> String {
    let artistes = artistes
        .iter()
        .enumerate()
        .map(|(index, artist_name)| match index + 1 == len {
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
