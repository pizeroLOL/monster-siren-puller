use crate::{
    download::{ download_songs, head_download, write_info},
    types::{Album, AlbumIndex, Response},
};
use futures::future::join_all;
use std::{
    error::Error,
    fs::{create_dir_all, read_dir},
    path::Path,
};

pub async fn get_cids() -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let t: Vec<AlbumIndex> = Response::get(&AlbumIndex::get_url()).await?;
    let t: Vec<(String, String)> = t
        .iter()
        .map(|x| x.get_cid().to_string())
        .zip(t.iter().map(|y| y.get_name().to_string()))
        .collect();
    Ok(t)
    // println!("{:?}",download_map);
}

pub fn get_errs(about: &str, tasks: Vec<Result<(), Box<dyn Error>>>) -> Result<(), Box<dyn Error>> {
    let tasks = tasks
        .into_iter()
        .filter_map(|x| x.err())
        .collect::<Vec<_>>();
    match tasks.is_empty() {
        true => Ok(()),
        false => {
            let e = format!("{about} : {tasks:#?}");
            Err(e.into())
        }
    }
}

/// 以并行的方式获取所有的的专辑
pub async fn dont_use_download_all() -> Result<(), Box<dyn Error>> {
    let dir = Path::new("./siren");
    let download_map = get_cids().await?;
    let dl_tasks: Vec<_> = download_map
        .iter()
        .map(|(cid, name)| download_album(cid, dir, name))
        .collect();
    let dl_tasks = join_all(dl_tasks).await;
    get_errs("download album error", dl_tasks)?;
    Ok(())
}

/// 下载前几个的专辑
///
/// top：下载的数量
pub async fn download_top(dir: &Path, top: usize) -> Result<(), Box<dyn Error>> {
    let download_map = get_cids().await?;
    let tasks = download_map
        .iter()
        .enumerate()
        .filter(|(index, _)| index < &top)
        .map(|(_, key_value)| key_value);
    for (cid, dir_name) in tasks {
        download_album(cid, dir, dir_name).await?
    }
    Ok(())
}

/// 下载缺失的专辑
pub async fn download_sync(dir: &Path) -> Result<(), Box<dyn Error>> {
    if !dir.try_exists()? {
        create_dir_all(dir)?
    }
    let dirs = read_dir(dir)?
        .flatten()
        .map(|x| x.path().to_string_lossy().to_string())
        .collect::<Vec<String>>();
    let download_map = get_cids().await?;
    for (cid, dir_name) in download_map {
        if dirs.contains(&dir_name.trim().to_string()) {
            println!("skip {}", dir_name);
            continue;
        }
        download_album(&cid, dir, &dir_name).await?;
    }
    Ok(())
}

/// 以遍历的方式下载所有专辑
pub async fn download_all(dir: &Path) -> Result<(), Box<dyn Error>> {
    let download_map = get_cids().await?;
    for (cid, dir_name) in download_map {
        download_album(&cid, dir, &dir_name).await?;
    }
    Ok(())
}

/// # 下载所有专辑
///
/// ## 参数
///
/// - cid：专辑编号
/// - dir：专辑文件夹所在的地址
/// - dir_name：专辑名称
pub async fn download_album(cid: &str, dir: &Path, dir_name: &str) -> Result<(), Box<dyn Error>> {
    let data: Album = Response::get(&Album::get_url(cid)).await?;
    println!("start {}", data.get_name());
    let dir = &dir.join(dir_name.trim());
    create_dir_all(dir)?;
    let dl_headimg_tasks = vec![
        head_download(data.get_cover_url(), "head.", dir),
        head_download(data.get_cover_de_url(), "wide_head.", dir),
    ];
    let dl_headimg_tasks = join_all(dl_headimg_tasks).await;
    get_errs("download head image error", dl_headimg_tasks)?;
    download_songs(&data, dir).await?;
    write_info(&data, &dir.join("info.txt")).await?;
    println!("end {}", data.get_name());
    Ok(())
}
