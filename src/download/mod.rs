mod interface;

pub mod info;

pub use interface::*;

use crate::{
    types::{Album, Response as SirenResponse, Song, SongIndex},
    USER_AGENT,
};
use futures::future;
use reqwest::Response;
use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
    path::Path,
    thread,
    time::Duration,
};

use self::info::format_info;

pub async fn download(url: &str) -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .timeout(Duration::from_secs(30))
        .build()?;
    let mut t = client.get(url).send().await;
    for _ in 0..3 {
        if let Ok(o) = t {
            return Ok(o);
        }
        thread::sleep(Duration::from_secs(5));
        t = client.get(url).send().await;
        continue;
    }
    t
}

/// # 下载专辑头图
///
/// ## 参数
///
/// - url：专辑地址
/// - name：重命名的名称（不包括后缀）
/// - dir：专辑的地址
pub async fn head_download(url: &str, name: &str, dir: &Path) -> Result<(), Box<dyn Error>> {
    let t = url.split('.').rev().collect::<Vec<&str>>();
    let t = t.first().unwrap();
    let file = dir.join(name.to_owned() + t);
    download_file(url, &file).await?;
    Ok(())
}

async fn get_file(url: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let x = download(url)
        .await?
        .bytes()
        .await?
        .bytes()
        .collect::<Result<Vec<u8>, _>>()?;
    Ok(x)
}

async fn download_file(url: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    let mut errors = String::new();
    let mut file = File::create(path)?;
    for i in 0..3 {
        match get_file(url).await {
            Ok(o) => {
                file.write_all(&o)?;
                return Ok(());
            }
            Err(e) => errors += &format!("[{i}] {e}\n"),
        }
    }
    Err(errors.into())
}

/// # 写入 info
///
/// ## 参数
///
/// - data：传入专辑类型
/// - path：文件的地址
pub async fn write_info(data: &Album, path: &Path) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;
    file.write_all(&format_info(data).bytes().collect::<Vec<u8>>())?;
    Ok(())
}

/// # 下载专辑内的音乐音乐
///
/// ## 参数
///
/// - data：专辑信息
/// - path：专辑文件夹地址
pub async fn download_songs(data: &Album, path: &Path) -> Result<(), Box<dyn Error>> {
    let mut tasks = Vec::new();
    for x in data.get_songs() {
        tasks.push(download_song(x, path));
    }
    let tasks = future::join_all(tasks).await;
    let name = data.get_name();
    let err_about = format!("download {name} error");
    get_errs(&err_about, tasks)?;
    Ok(())
}

/// # 下载单首音乐
///
/// ## 参数
///
/// - index：SongIndex，拿到地址
/// - path：专辑文件夹地址
async fn download_song(index: &SongIndex, path: &Path) -> Result<(), Box<dyn Error>> {
    let cid = Song::get_url(index.get_cid());
    let song: Song = SirenResponse::get(&cid).await?;
    let name = song.get_name();
    println!("  start:{}", name);
    let t = [
        song.get_source_url(),
        song.get_lyric_url(),
        song.get_mv_url(),
        song.get_mv_cover_url(),
    ];
    let mut tasks = Vec::new();
    for i in t.iter().filter(|t| t.is_some()) {
        tasks.push(download_asset(i, path, &song))
    }
    let tasks = future::join_all(tasks).await;
    let about = format!("download {name} assets error");
    get_errs(&about, tasks)?;
    println!("  end:{}", song.get_name());
    Ok(())
}

async fn download_asset(
    item: &Option<String>,
    path: &Path,
    song: &Song,
) -> Result<(), Box<dyn Error>> {
    let i = item.clone().unwrap();
    let t = i.split('.').rev().collect::<Vec<&str>>();
    download_file(
        &i,
        &path.join(song.get_name().trim().to_owned() + "." + t.first().unwrap()),
    )
    .await?;
    Ok(())
}
