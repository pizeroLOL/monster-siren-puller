pub mod download;
pub mod types;

use std::{
    error::Error,
    fs::{self, read_dir},
    path::Path,
};

use types::{Album, AlbumIndex, Song};

static API: &str = "https://monster-siren.hypergryph.com/api/";
static USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/112.0";

/// 用于删除写入了一半的专辑
///
/// 原理是清除没有 info.txt 的专辑文件夹
///
/// ```rust
/// use std::{path::Path,fs};
/// use monster_siren_puller::repair;
///
/// let dir = Path::new("./siren/NotDownloadFinishAlbum/");
/// fs::create_dir_all(dir).unwrap();
/// repair().unwrap();
/// assert!(!dir.exists());
///
/// ```
pub fn repair(dir: &Path) -> Result<(), Box<dyn Error>> {
    let dirs = read_dir(dir)?
        .filter_map(|p| {
            let dir = p.expect("无法读取文件夹").path();
            let file = dir.join("info.txt");
            if file.try_exists().expect("无法读取文件") {
                None
            } else {
                Some(dir)
            }
        })
        .collect::<Vec<_>>();
    for i in dirs {
        fs::remove_dir_all(i.to_str().expect("cover_err"))?
    }

    Ok(())
}

pub async fn online_repair(dir: &Path) -> Result<(), Box<dyn Error>> {
    let tasks = AlbumIndex::get().await?;
    let tasks = tasks
        .iter()
        .map(|x| (x.get_cid(), x.get_name().trim()))
        .filter(|(_, name)| dir.join(name).is_dir());
    // let mut albums = vec![];
    for (cid, name) in tasks {
        match check_album(cid, name, dir).await? {
            Some(o) => println!("{o}"),
            None => (),
        }
    }
    Ok(())
}

async fn check_album<'a>(
    cid: &'a str,
    name: &'a str,
    dir: &'a Path,
) -> Result<Option<&'a str>, Box<dyn Error>> {
    if !dir.join(name).join("info.txt").is_file() {
        println!("{name}")
    }
    let album = Album::get(cid).await?;
    if let Some(_) = check_head_img(&album, name, dir) {
        return Ok(Some(name));
    }
    let songs = album.get_songs().iter().map(|x| Song::get(x.get_cid()));
    let x = futures::future::join_all(songs)
        .await
        .iter()
        .filter_map(|x| match x {
            Ok(o) => Some(o),
            Err(e) => {
                eprintln!("check {name}'s song err : {e}");
                None
            }
        })
        .map(|x| {
            [
                x.get_lyric_url(),
                x.get_mv_cover_url(),
                x.get_mv_url(),
                x.get_source_url(),
            ]
            .into_iter()
            .flatten()
            .map(|x|x.)
            .collect::<Vec<_>>()
        });
    Ok(None)
}

fn check_head_img(album: &Album, name: &str, dir: &Path) -> Option<()> {
    let tasks = [
        ("head.", album.get_cover_url()),
        ("wide_head.", album.get_cover_de_url()),
    ]
    .iter()
    .map(|(x, y)| x.to_owned().to_owned() + get_url_name(y).unwrap_or_default())
    .map(|x| dir.join(name).join(x))
    .map(|x| x.is_file())
    .collect::<Vec<_>>();
    if tasks.contains(&false) {
        return Some(());
    };
    None
}

fn get_url_name(url: &str) -> Option<&str> {
    url.split('.').next_back()
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::online_repair;

    #[tokio::test]
    async fn online_repair_check() {
        online_repair(Path::new("/run/media/pizero/Ventoy/fish/siren"))
            .await
            .unwrap()
    }
}
