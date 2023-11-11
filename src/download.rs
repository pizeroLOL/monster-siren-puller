use crate::{
    types::{Album, Song, SongIndex},
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

fn get_errs(about: &str, tasks: Vec<Result<(), Box<dyn Error>>>) -> Result<(), Box<dyn Error>> {
    let tasks = tasks.iter().filter(|x| x.is_err()).collect::<Vec<_>>();
    if tasks.is_empty() {
        return Ok(());
    };
    let tasks = tasks
        .iter()
        .filter_map(|d| match d {
            Ok(_) => None,
            Err(e) => Some(e),
        })
        .collect::<Vec<_>>();
    Err(format!("{about} : {tasks:#?}").into())
}

pub async fn download(url: &str) -> Result<Response, Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .timeout(Duration::from_secs(30))
        .build()?;
    let mut t = client.get(url).send().await;
    for _ in 0..3 {
        match t {
            Ok(it) => return Ok(it),
            Err(_) => {
                thread::sleep(Duration::from_secs(5));
                t = client.get(url).send().await;
                continue;
            }
        }
    }
    Ok(t?)
}

/// 获取所有专辑的 cid

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

async fn download_file(url: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    async fn get_file(url: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let x = download(url)
            .await?
            .bytes()
            .await?
            .bytes()
            .collect::<Result<Vec<u8>, _>>()?;
        Ok(x)
    }
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

/// # 写入 info
///
/// ## 参数
///
/// - data：传入专辑类型
/// - path：文件的地址
pub async fn write_info(data: &Album, path: &Path) -> Result<(), Box<dyn Error>> {
    let songs = data.get_songs();
    let t_max = songs.len();
    let t = songs
        .iter()
        .map(|x| format_song_artistes(x.get_name(), x.get_artistes(), t_max))
        .collect::<String>();
    let t = format!(
        "专辑名：{}\n简介：{}\n{}",
        data.get_name(),
        data.get_intro(),
        t
    )
    .bytes()
    .collect::<Vec<u8>>();
    let mut file = File::create(path)?;
    file.write_all(&t)?;
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
    let song = Song::get(index.get_cid()).await?;
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

#[cfg(test)]
mod test {
    use crate::USER_AGENT;

    use super::download_file;
    use std::{path::Path, thread, time::Duration};

    #[tokio::test]
    async fn t() {
        let path = "https://web.hycdn.cn/siren/pic/20230427/840c552b50612166caa8ee52ac7f6654.jpg";
        download_file(path, Path::new("./hi.jpg")).await.unwrap()
    }
    #[tokio::test]
    async fn x() {
        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap();
        let mut t = client.get("http://127.0.0.1:8000").send().await;
        let mut tmp = 0;
        let count = 3;
        while t.is_err() && count > tmp {
            println!("testing");
            tmp += 1;
            t = client.get("http://127.0.0.1:8000").send().await;
            thread::sleep(Duration::from_secs(1))
        }
        println!("{:?}", t)
    }
}
