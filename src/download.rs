use futures::future;
use reqwest::Response;
use std::io::Read;
use std::{error::Error, fs, path::Path};
use std::{fs::File, io::Write, thread, time::Duration};

use crate::{
    album_detail::{get_album, Album, SongIndex},
    albums::get_albums_index,
    song::{get_song, Song},
    USER_AGENT,
};

fn get_errs(about: &str, tasks: Vec<Result<(), Box<dyn Error>>>) -> Result<(), Box<dyn Error>> {
    let tasks = tasks.iter().filter(|x| x.is_err()).collect::<Vec<_>>();
    if tasks.is_empty() {
        return Ok(());
    };
    let tasks = tasks
        .iter()
        .map(|d| {
            if let Err(e) = d {
                return Some(e);
            }
            None
        })
        .filter_map(|f| f)
        .collect::<Vec<_>>();
    Err(format!("{about} : {tasks:#?}").into())
}

pub async fn download(url: &str) -> Result<Response, Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .timeout(Duration::from_secs(30))
        .build()?;
    let mut t = client.get(url).send().await;
    for i in 0..3 {
        match t {
            Ok(it) => return Ok(it),
            Err(err) => {
                if i >= 2 {
                    return Err(err.into());
                }
                t = client.get(url).send().await;
                thread::sleep(Duration::from_secs(5));
                continue;
            }
        }
    }
    return Err("逻辑错误".into());
}

/// 获取所有专辑的 cid
pub async fn get_cids() -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let t = get_albums_index().await?;
    let t = t.to_index_list();
    let t: Vec<(String, String)> = t
        .iter()
        .map(|x| x.get_cid().to_string())
        .zip(t.iter().map(|y| y.get_name().to_string()))
        .collect();
    Ok(t)
    // println!("{:?}",download_map);
}

/// 以并行的方式获取所有的的专辑
pub async fn dont_use_download_all() -> Result<(), Box<dyn Error>> {
    let dir = Path::new("./siren");
    let download_map = get_cids().await?;
    let dl_tasks: Vec<_> = download_map
        .iter()
        .map(|(cid, name)| download_album(cid, dir, name))
        .collect();
    let dl_tasks = future::join_all(dl_tasks).await;
    get_errs("download album error", dl_tasks)?;
    Ok(())
}

/// 下载前几个的专辑
///
/// top：下载的数量
pub async fn download_top(top: usize) -> Result<(), Box<dyn Error>> {
    let dir = Path::new("./siren");
    let download_map = get_cids().await?;
    for (cid, dir_name) in download_map
        .iter()
        .enumerate()
        .filter(|x| x.0 < top)
        .map(|x| x.1)
    {
        download_album(cid, dir, dir_name).await?
    }
    Ok(())
}

/// 下载缺失的专辑
pub async fn download_sync() -> Result<(), Box<dyn Error>> {
    let dir = Path::new("./siren");
    if !dir.try_exists()? {
        fs::create_dir_all(dir)?
    }
    let dirs = fs::read_dir(dir)?
        .map(|x| {
            let x = x.expect("无法读取文件夹").path();
            let x = x
                .strip_prefix("./siren/")
                // TODO 添加错误提示
                .expect(&format!("删除前缀错误"));
            x.to_string_lossy().into()
        })
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
pub async fn download_all() -> Result<(), Box<dyn Error>> {
    let download_map = get_cids().await?;
    let dir = Path::new("./siren");
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
    let data = get_album(cid).await?;
    let data = data.to_album();
    println!("start {}", data.get_name());
    let dir = &dir.join(dir_name.trim());
    fs::create_dir_all(dir)?;
    let dl_headimg_tasks = vec![
        head_download(data.get_cover_url(), "head.", dir),
        head_download(data.get_cover_de_url(), "wide_head.", dir),
    ];
    let dl_headimg_tasks = future::join_all(dl_headimg_tasks).await;
    get_errs("download head image error", dl_headimg_tasks)?;
    download_songs(data, dir).await?;
    write_info(data, &dir.join("info.txt")).await?;
    println!("end {}", data.get_name());
    Ok(())
}

/// # 下载专辑头图
///
/// ## 参数
///
/// - url：专辑地址
/// - name：重命名的名称（不包括后缀）
/// - dir：专辑的地址
async fn head_download(url: &str, name: &str, dir: &Path) -> Result<(), Box<dyn Error>> {
    let t = url.split('.').rev().collect::<Vec<&str>>();
    let t = t.first().unwrap();
    let file = dir.join(name.to_owned() + t);
    download_file(url, &file).await?;
    Ok(())
}

async fn download_file(url: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    let byte = download(url)
        .await?
        .bytes()
        .await?
        .bytes()
        .collect::<Result<Vec<_>, _>>()?;

    let mut file = File::create(path)?;
    file.write_all(&byte)?;
    Ok(())
}

/// # 写入 info
///
/// ## 参数
///
/// - data：传入专辑类型
/// - path：文件的地址
async fn write_info(data: &Album, path: &Path) -> Result<(), Box<dyn Error>> {
    let t_max = data.get_songs().len() - 1;
    let t = data
        .get_songs()
        .iter()
        .map(|x| {
            let t = x
                .get_artistes()
                .iter()
                .enumerate()
                .map(|x| {
                    if x.0 == t_max {
                        x.1.to_owned()
                    } else {
                        x.1.to_owned() + "、"
                    }
                })
                .collect::<String>();
            format!("歌曲：{}\t作者：{}\n", x.get_name(), t)
        })
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
async fn download_songs(data: &Album, path: &Path) -> Result<(), Box<dyn Error>> {
    let mut tasks = Vec::new();
    for x in data.get_songs().iter() {
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
    let song = get_song(index.get_cid()).await?;
    let song = song.to_song();
    let name = song.get_name();
    println!("  start:{}", name);
    let t = vec![
        song.get_source_url(),
        song.get_lyric_url(),
        song.get_mv_url(),
        song.get_mv_cover_url(),
    ];
    let mut tasks = Vec::new();
    for i in t.iter().filter(|t| t.is_some()) {
        tasks.push(download_asset(i, path, song))
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
    use tokio::runtime::Builder;
    #[test]
    fn t() {
        let runtime = Builder::new_multi_thread().enable_all().build().unwrap();
        runtime
            .block_on(async {
                download_file(
                    "https://web.hycdn.cn/siren/pic/20230427/840c552b50612166caa8ee52ac7f6654.jpg",
                    Path::new("./hi.jpg"),
                )
                .await
            })
            .unwrap();
    }
    #[test]
    fn x() {
        let runtime = Builder::new_multi_thread().enable_all().build().unwrap();
        runtime.block_on(async {
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
        });
    }
}
