use std::{
    error::Error,
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::Path,
    time::Duration,
};

use futures::future::join_all;
use reqwest::Response;
use tokio::time::sleep;

use super::{config::DLConfig, padding::DLTask};

pub async fn download(url: &str, ua: &str, timeout: Duration) -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::builder()
        .user_agent(ua)
        .timeout(timeout)
        .build()?;
    let mut t = client.get(url).send().await;
    for _ in 0..3 {
        if let Ok(o) = t {
            return Ok(o);
        }
        sleep(timeout).await;
        t = client.get(url).send().await;
        continue;
    }
    t
}

pub fn create_dirs(dir: &Path, tasks: &[DLTask]) -> std::io::Result<()> {
    let mut task_dirs = tasks.iter().map(|x| x.dir(dir)).collect::<Vec<_>>();
    task_dirs.dedup();
    for path in task_dirs {
        if path.exists() {
            continue;
        }
        create_dir_all(path)?;
    }
    Ok(())
}

pub async fn get_file(url: &str, ua: &str, timeout: Duration) -> Result<Vec<u8>, Box<dyn Error>> {
    let x = download(url, ua, timeout)
        .await?
        .bytes()
        .await?
        .bytes()
        .collect::<Result<Vec<u8>, _>>()?;
    Ok(x)
}

pub async fn download_file(task: &DLTask, config: &DLConfig) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    let mut file = File::create(task.path(&config.dir)).map_err(|e| vec![e.to_string()])?;
    for _ in 0..3 {
        match get_file(&task.url, &config.ua, config.timeout).await {
            Ok(o) => {
                file.write_all(&o).map_err(|e| vec![e.to_string()])?;
                println!("{} {} ok", task.album, task.asset);
                return Ok(());
            }
            Err(e) => errors.push(e.to_string()),
        }
    }
    Err(errors.into())
}

pub async fn download_tasks(tasks: &[DLTask], config: &DLConfig) -> Result<(), Box<dyn Error>> {
    for chunk in tasks.chunks(config.thread) {
        let task = chunk
            .iter()
            .map(|i| download_file(i, config))
            .collect::<Vec<_>>();
        let tmp = join_all(task)
            .await
            .into_iter()
            .filter_map(|e| e.err())
            .flatten()
            .collect::<Vec<_>>();
        if tmp.len() != 0 {
            return Err(format!("{:#?}", tmp).into());
        }
    }
    Ok(())
}
