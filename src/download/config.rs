use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use crate::USER_AGENT;

#[derive(Debug, Clone, Default)]
pub struct DLConfigBuilder {
    dir: Option<PathBuf>,
    thread: Option<usize>,
    ua: Option<String>,
    timeout: Option<Duration>,
    retry_time: Option<Duration>,
}

impl DLConfigBuilder {
    pub fn new() -> Self {
        Self {
            dir: None,
            thread: None,
            ua: None,
            timeout: None,
            retry_time: None,
        }
    }
    pub fn dir(self, dir_path: &Path) -> Self {
        Self {
            dir: Some(dir_path.to_path_buf()),
            ..self
        }
    }
    pub fn thread(self, count: usize) -> Self {
        Self {
            thread: Some(count),
            ..self
        }
    }
    pub fn ua(self, text: &str) -> Self {
        Self {
            ua: Some(text.to_string()),
            ..self
        }
    }
    pub fn timeout(self, dur: Duration) -> Self {
        Self {
            timeout: Some(dur),
            ..self
        }
    }
    pub fn retry_time(self, dur: Duration) -> Self {
        Self {
            retry_time: Some(dur),
            ..self
        }
    }
    pub fn build(self) -> DLConfig {
        let def = DLConfig::default();
        DLConfig {
            dir: self.dir.unwrap_or(def.dir),
            thread: self.thread.unwrap_or(def.thread),
            ua: self.ua.unwrap_or(def.ua),
            timeout: self.timeout.unwrap_or(def.timeout),
            retry_time: self.retry_time.unwrap_or(def.retry_time),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DLConfig {
    pub dir: PathBuf,
    pub thread: usize,
    pub ua: String,
    pub timeout: Duration,
    pub retry_time: Duration,
}

impl DLConfig {
    pub fn new(
        dir: PathBuf,
        thread: usize,
        ua: String,
        timeout: Duration,
        retry_time: Duration,
    ) -> Self {
        Self {
            dir,
            thread,
            ua,
            timeout,
            retry_time,
        }
    }
}

impl Default for DLConfig {
    fn default() -> Self {
        Self {
            dir: Path::new("siren").to_path_buf(),
            thread: 1,
            ua: USER_AGENT.to_owned(),
            timeout: Duration::from_secs(30),
            retry_time: Duration::from_secs(30),
        }
    }
}
