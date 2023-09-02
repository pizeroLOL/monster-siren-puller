use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    // /// 将日志调整到 Debug 模式
    // #[arg(long, value_name = "BOOL")]
    // pub debug: Option<bool>,
    /// 下载到指定文件夹
    #[arg(short, long, value_name = "DIR_PATH")]
    pub dir: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
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

#[derive(Subcommand)]
pub enum AlbumCommand {
    /// 关于该专辑
    About,

    /// 显示专辑内的音乐及其 cid
    Show,

    /// 获取该专辑
    Get,
}
