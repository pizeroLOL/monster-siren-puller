use futures::future;
use reqwest::Response;
use std::{fs::File, io::Write, thread, time::Duration};

use crate::{
    album_detail::{get_album, Album},
    albums::get_albums_index,
    song::get_song,
    USER_AGENT,
};
use std::{collections::HashMap, error::Error, fs, io::Read, path::Path};

pub async fn download(url: &str) -> Result<Response, Box<dyn Error>> {
    let client = reqwest::Client::builder().user_agent(USER_AGENT).build()?;
    let mut t = client.get(url).send().await;
    while t.is_err() {
        println!("testing");
        t = client.get(url).send().await;
        thread::sleep(Duration::from_secs(1))
    };
    Ok(t?)
}

pub async fn download_all() -> Result<(), Box<dyn Error>> {
    let t = get_albums_index().await?.to_index_list();
    let download_map: HashMap<_, _> = t
        .iter()
        .map(|x| &x.cid)
        .zip(t.iter().map(|y| &y.name))
        .collect();
    let dir = Path::new("./siren");
    let mut tasks = Vec::new();
    for (cid, dir_name) in download_map {
        tasks.push(download_album(cid, dir, dir_name))
    }
    future::join_all(tasks).await;
    Ok(())
}

async fn download_album(cid: &str, dir: &Path, dir_name: &str) -> Result<(), Box<dyn Error>> {
    let data = get_album(cid).await?.to_album();
    println!("start {}", data.name);
    let dir = &dir.join(dir_name.trim());
    fs::create_dir_all(dir)?;
    println!("start head {}", data.name);
    head_download(&data.coverUrl, "head.", dir).await?;
    head_download(&data.coverDeUrl, "wide_head.", dir).await?;
    write_info(&data, &dir.join("info.txt")).await?;
    println!("start song {}", data.name);
    download_song(&data, dir).await?;
    Ok(())
}

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
        .collect::<Result<Vec<_>, _>>();
    let mut file = File::create(path)?;
    file.write_all(&byte?)?;
    Ok(())
}

async fn write_info(data: &Album, path: &Path) -> Result<(), Box<dyn Error>> {
    let t_max = data.songs.len() - 1;
    let t = data
        .songs
        .iter()
        .map(|x| {
            let t = x
                .artistes
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
            format!("歌曲：{}\t作者：{}\n", x.name, t)
        })
        .collect::<String>();
    let t = format!("专辑名：{}\n简介：{}\n{}", data.name, data.intro, t)
        .bytes()
        .collect::<Vec<u8>>();
    let mut file = File::create(path)?;
    file.write_all(&t)?;
    Ok(())
}

async fn download_song(data: &Album, path: &Path) -> Result<(), Box<dyn Error>> {
    for x in data.songs.iter() {
        let song = get_song(&x.cid).await?.to_song();
        let t = song.sourceUrl.split('.').rev().collect::<Vec<&str>>();
        download_file(
            &song.sourceUrl,
            &path.join(song.name.trim().to_owned() + "." + t.first().unwrap()),
        )
        .await?;
        let t = vec![song.lyricUrl, song.mvUrl, song.mvCoverUrl];
        for i in t.iter().filter(|t| t.is_some()) {
            let i = i.clone().unwrap();
            let t = i.split('.').rev().collect::<Vec<&str>>();
            download_file(
                &i,
                &path.join(song.name.trim().to_owned() + "." + t.first().unwrap()),
            )
            .await?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::USER_AGENT;

    use super::download_file;
    use std::{
        path::Path,
        time::Duration, thread,
    };
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
