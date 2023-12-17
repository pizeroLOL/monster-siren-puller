pub mod build_info;
pub mod config;
pub mod downloading;
pub mod padding;

use self::{
    build_info::write_infos,
    config::DLConfig,
    downloading::{create_dirs, download_tasks},
    padding::from_album_indexes,
};
use crate::types::{AlbumIndex, Response as SirenRespons};

use std::{
    error::Error,
    fs::{create_dir_all, read_dir, DirEntry},
    path::Path,
};

static REPLACE: [char; 9] = ['\\', '/', '*', ':', '"', '<', '>', '|', '.'];

pub async fn download_sync(config: &DLConfig) -> Result<(), Box<dyn Error>> {
    #[inline]
    fn get_dir_name(dir: DirEntry, root: &Path) -> std::string::String {
        dir.path()
            .strip_prefix(root)
            .unwrap()
            .to_string_lossy()
            .to_string()
    }
    if !config.dir.try_exists()? {
        create_dir_all(&config.dir)?
    }
    let dirs = read_dir(&config.dir)?
        .flatten()
        .map(|now| get_dir_name(now, &config.dir))
        .collect::<Vec<String>>();
    println!("get dirs OK");
    // println!("{dirs:#?}");
    let url = AlbumIndex::get_url();
    let album_indexes = SirenRespons::<Vec<AlbumIndex>>::get(&url, config).await?;
    let album_indexes = album_indexes
        .into_iter()
        .filter(|x| !dirs.contains(&x.get_name().replace(REPLACE, "").trim().to_owned()))
        .collect::<Vec<_>>();
    // println!("{album_indexes:#?}");
    let (tasks, albums) = from_album_indexes(&album_indexes, config).await?;
    println!("padding OK");
    create_dirs(&config.dir, tasks)?;
    download_tasks(tasks, config).await?;
    println!("download OK");
    write_infos(albums, config.dir.as_path())?;
    println!("gen info OK");
    Ok(())
}

pub async fn download_top(index: usize, config: &DLConfig) -> Result<(), Box<dyn Error>> {
    if index == 0 {
        println!("前 0 个默认下载全部专辑");
        return download_all(config).await;
    }
    let url = AlbumIndex::get_url();
    let album_indexes = &SirenRespons::<Vec<AlbumIndex>>::get(&url, config).await?;
    let Some(album_indexes) = album_indexes.chunks(index).next() else {
        println!("不足 {} 个，默认下载全部", index);
        return download_all(config).await;
    };
    let (tasks, albums) = from_album_indexes(album_indexes, config).await?;
    println!("padding OK");
    create_dirs(&config.dir, tasks)?;
    download_tasks(tasks, config).await?;
    println!("download OK");
    write_infos(albums, config.dir.as_path())?;
    println!("gen info OK");
    Ok(())
}

pub async fn download_all(config: &DLConfig) -> Result<(), Box<dyn Error>> {
    let url = AlbumIndex::get_url();
    let album_indexes = SirenRespons::<Vec<AlbumIndex>>::get(&url, config).await?;
    let (tasks, albums) = from_album_indexes(&album_indexes, config).await?;
    println!("padding OK");
    create_dirs(&config.dir, tasks)?;
    download_tasks(tasks, config).await?;
    println!("download OK");
    write_infos(albums, config.dir.as_path())?;
    println!("gen info OK");
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::download3::{config::DLConfigBuilder, download_all};
    use std::path::Path;
    use std::time::Duration;

    #[tokio::test]
    async fn test_download_all() {
        let config = DLConfigBuilder::new()
            .timeout(Duration::from_secs(10))
            .ua("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:120.0) Gecko/20100101 Firefox/120.0")
            .build();
        download_all(&config).await.unwrap();
    }
    #[test]
    fn testing() {
        let path = Path::new("target");
        println!("{} \t {}", path.is_dir(), path.exists());
    }
}
