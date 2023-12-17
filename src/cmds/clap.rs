use clap::{Parser, Subcommand};
use monster_siren_puller::download3::config::{DLConfig, DLConfigBuilder};
use std::{path::PathBuf, time::Duration};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    // /// 将日志调整到 Debug 模式
    // #[arg(long, value_name = "BOOL")]
    // pub debug: Option<bool>,
    /// 下载到指定文件夹
    #[arg(short, long, value_name = "DIR_PATH")]
    pub dir: Option<PathBuf>,

    #[arg(short = 'T', long, value_name = "THREAD")]
    pub thread: Option<usize>,

    #[arg(short, long, value_name = "User-Agent")]
    pub ua: Option<String>,

    #[arg(short, long, value_name = "TIMEOUT")]
    pub timeout: Option<f64>,

    #[arg(short, long, value_name = "RETRY-TIME")]
    pub retry_time: Option<f64>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone)]
pub enum ToDLConfigError {
    NegativeNumber,
}

impl TryInto<DLConfig> for Cli {
    type Error = ToDLConfigError;

    fn try_into(self) -> Result<DLConfig, Self::Error> {
        if let Some(num) = self.timeout {
            if num <= 0.0 {
                return Err(ToDLConfigError::NegativeNumber);
            }
        }
        let def = DLConfig::default();
        let tmp = DLConfigBuilder::new()
            .dir(&self.dir.unwrap_or(def.dir))
            .ua(&self.ua.unwrap_or(def.ua))
            .thread(self.thread.unwrap_or(def.thread))
            .timeout(
                self.timeout
                    .map(Duration::from_secs_f64)
                    .unwrap_or(def.timeout),
            )
            .retry_time(
                self.retry_time
                    .map(Duration::from_secs_f64)
                    .unwrap_or(def.retry_time),
            )
            .build();
        Ok(tmp)
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// 获取前 <INDEX> 个专辑
    Top { index: usize },

    /// 获取所有专辑
    All,

    /// 获取未下载专辑
    Sync,

    /// 删除下载了一半的专辑
    Repair,

    /// 展示专辑 cid 和对应名称
    Show,

    /// 显示专辑相关功能，比如关于、获取、显示专辑内音乐及其 cid
    Album {
        cid: usize,

        #[command(subcommand)]
        command: AlbumCommand,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum AlbumCommand {
    /// 关于该专辑
    About,

    /// 显示专辑内的音乐及其 cid
    Show,

    /// 获取该专辑
    Get,
}
