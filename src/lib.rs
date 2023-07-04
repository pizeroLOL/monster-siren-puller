use std::{
    error::Error,
    fs::{self, read_dir},
    path::Path,
};

pub mod album_detail;
pub mod albums;
pub mod download;
pub mod song;

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
