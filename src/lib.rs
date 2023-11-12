use std::{
    error::Error,
    fs::{self, read_dir},
    path::Path,
};

pub mod download;
pub mod types;

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
        .filter_map(|dir| dir.ok())
        .map(|dir| dir.path().join("info.txt"));
    for i in dirs {
        i.try_exists().map_err(|e| format!("文件不存在：{e}"))?;
        let path = i
            .to_str()
            .ok_or(format!("删除错误：{}", i.to_string_lossy()))?;
        fs::remove_dir_all(path)?;
    }

    Ok(())
}

static API: &str = "https://monster-siren.hypergryph.com/api/";
static USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/112.0";
