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
}

impl DLConfigBuilder {
    pub fn new() -> Self {
        Self {
            dir: None,
            thread: None,
            ua: None,
            timeout: None,
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
    pub fn build(self) -> DLConfig {
        let def = DLConfig::default();
        DLConfig {
            dir: self.dir.unwrap_or(def.dir),
            thread: self.thread.unwrap_or(def.thread),
            ua: self.ua.unwrap_or(def.ua),
            timeout: self.timeout.unwrap_or(def.timeout),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DLConfig {
    pub dir: PathBuf,
    pub thread: usize,
    pub ua: String,
    pub timeout: Duration,
}


impl DLConfig {
    pub fn new(dir: PathBuf, thread: usize, ua: String, timeout: Duration) -> Self {
        Self {
            dir,
            thread,
            ua,
            timeout,
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
        }
    }
}
