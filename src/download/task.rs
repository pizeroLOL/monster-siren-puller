use std::{path::PathBuf, time::Duration};

#[derive(Debug, Clone)]
pub struct Task {
    pub path: PathBuf,
    pub thread: usize,
    pub ua: String,
    pub timeout: Duration,
}
