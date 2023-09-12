pub mod album;
pub mod downlod;
pub mod get;
pub mod write;

use std::{
    path::{Path, PathBuf},
    time::Duration,
};

const REPLACE: [char; 6] = ['\\', '/', '.', '?', '*', ':'];

#[derive(Debug, Clone)]
pub struct Task<'a> {
    path: PathBuf,
    thread: usize,
    ua: &'a str,
    timeout: Duration,
}

#[derive(Debug)]
pub enum TaskError {
    CreateDir(std::io::Error),
    CreateFile(std::io::Error),
    Deserialize(reqwest::Error),
    GenClient(reqwest::Error),
    NoneUrl(String),
    SendData(reqwest::Error),
    Write(std::io::Error),
    DownloadSongAssest(Vec<TaskError>),
    DownloadImage(Vec<TaskError>),
    DownloadSongs(Vec<TaskError>)
}

impl<'a> Task<'a> {
    pub fn new(path: &Path) -> Task<'a> {
        let ua = "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/112.0";
        let timeout = Duration::from_secs(30);
        Self {
            path: path.to_path_buf(),
            thread: 1,
            ua,
            timeout,
        }
    }

    pub fn set_thread(&self, thread: usize) -> Task<'a> {
        let mut x = self.clone();
        x.thread = thread;
        x
    }

    pub fn set_ua(&self, ua: &'a str) -> Task<'a> {
        let mut x = self.clone();
        x.ua = ua;
        x
    }

    pub fn set_path(&self, path: &Path) -> Task<'a> {
        let mut x = self.clone();
        x.path = path.to_path_buf();
        x
    }
}
