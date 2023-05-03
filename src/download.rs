use std::{fs::File, io::Write};

use reqwest::Response;

use crate::{
    album_detail::{get_album, Album},
    albums::get_albums_index,
    song::get_song,
    USER_AGENT,
};
use std::{collections::HashMap, error::Error, fs, io::Read, path::Path};

pub async fn download(url: &str) -> Result<Response, Box<dyn Error>> {
    let client = reqwest::Client::builder().user_agent(USER_AGENT).build()?;
    Ok(client.get(url).send().await?)
}

pub async fn download_all() -> Result<(), Box<dyn Error>> {
    let t = get_albums_index().await?.to_index_list();
    let download_map: HashMap<_, _> = t
        .iter()
        .map(|x| &x.cid)
        .zip(t.iter().map(|y| &y.name))
        .collect();
    let dir = Path::new("./siren");
    for (cid, dir_name) in download_map {
        let data = get_album(cid).await?.to_album();
        println!("start {}", data.name);
        let dir = &dir.join(dir_name.trim());
        fs::create_dir_all(dir)?;
        println!("start head {}", data.name);
        head_download(&data, "head.", dir).await?;
        head_download(&data, "wide_head.", dir).await?;
        write_info(&data, &dir.join("info.txt")).await?;
        println!("start song {}", data.name);
        download_song(&data, dir).await?;
    }
    Ok(())
}

async fn head_download(data: &Album, name: &str, dir: &Path) -> Result<(), Box<dyn Error>> {
    let t = data.coverUrl.split('.').rev().collect::<Vec<&str>>();
    let t = t.first().unwrap();
    let file = dir.join(name.to_owned() + t);
    download_file(&data.coverUrl, &file).await?;
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
    let t = data
        .songs
        .iter()
        .map(|x| {
            let t = x
                .artistes
                .iter()
                .map(|x| x.to_owned() + "、")
                .collect::<String>();
            format!("歌曲：{}\t作者：{}\n", x.name, t)
        })
        .collect::<String>();
    let t = format!("专辑名：{}\n简介：{}\n{}", data.name, data.intro, t)
        .bytes()
        .collect::<Vec<u8>>();
    let mut file = File::create(path)?;
    file.write(&t)?;
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
    use super::download_file;
    use std::path::Path;
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
}
