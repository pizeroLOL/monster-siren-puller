use std::{
    error::Error,
    fs::{self, read_dir},
    path::Path,
};

pub mod album_detail;
pub mod albums;
pub mod download;
pub mod song;

/// 用于删除写入了一半的专辑
///
/// 原理是清除没有 info.txt 的专辑文件夹
///
/// ```rust
/// use std::{path::Path,fs};
/// use monster_siren_puller::repair;
/// 
/// let dir = Path::new("./siren")
/// let full_dir = Path::new("./siren/NotDownloadFinishAlbum/");
/// fs::create_dir_all(full_dir).unwrap();
/// repair(dir).unwrap();
/// assert!(!dir.exists());
///
/// ```
pub fn repair() -> Result<(), Box<dyn Error>> {
    let path = Path::new("siren");
    let dirs = read_dir(path)?
        .map(|p| {
            let dir = p.expect("无法读取文件夹").path();
            let file = dir.join("info.txt");
            if file.try_exists().expect("无法读取文件") {
                None
            } else {
                Some(dir)
            }
        })
        .filter_map(|d| d)
        .collect::<Vec<_>>();
    for i in dirs {
        fs::remove_dir_all(i.to_str().expect("cover_err"))?
    }

    Ok(())
}

static API: &str = "https://monster-siren.hypergryph.com/api/";
static USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/112.0";
