mod cmds;
mod download_task;

use crate::{cmds::album::AlbumCmd, download_task::Task};
use ::clap::Parser;
use cmds::{
    clap::{AlbumCommand, Cli, Commands},
    try_or_eprintln::OkOrEPrintln,
};
use std::{
    error::Error,
    fs::{self, read_dir},
    path::Path,
};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let dir = cli.dir.unwrap_or(Path::new("./siren/").to_path_buf());
    let dir = dir.as_path();

    match cli.command {
        // Commands::Top { index } => download_top(dir, index).await.ok_or_eprintln(),
        Commands::All => {
            let x = Task::new(dir).download_all().await;
            println!("{x:?}")
        }
        // Commands::Sync => download_sync(dir).await.ok_or_eprintln(),
        Commands::Repair => repair(dir).ok_or_eprintln(),
        Commands::Show => to_show().await.ok_or_eprintln(),
        Commands::Album { cid, command } => album(dir, cid, command).await,
        _ => todo!(),
    }
}

/// 用于删除写入了一半的专辑
///
/// 原理是清除没有 info.txt 的专辑文件夹
///
/// ```rust
/// use std::{path::Path,fs};
/// use monster_siren_puller::repair;
///
/// let dir = Path::new("./siren/NotDownloadFinishAlbum/");
/// fs::create_dir_all(dir).unwrap();
/// repair().unwrap();
/// assert!(!dir.exists());
///
/// ```
fn repair(dir: &Path) -> Result<(), Box<dyn Error>> {
    let dirs = read_dir(dir)?
        .filter_map(|p| {
            let dir = p.expect("无法读取文件夹").path();
            let file = dir.join("info.txt");
            file.try_exists().expect("无法读取文件").then_some(dir)
        })
        .collect::<Vec<_>>();
    for i in dirs {
        fs::remove_dir_all(i.to_str().expect("cover_err"))?
    }

    Ok(())
}

async fn album(dir: &Path, cid: usize, cmd: AlbumCommand) {
    match cmd {
        AlbumCommand::About => AlbumCmd::about(cid).await.ok_or_eprintln(),
        AlbumCommand::Show => AlbumCmd::show(cid).await.ok_or_eprintln(),
        AlbumCommand::Get => AlbumCmd::get(dir, cid).await.ok_or_eprintln(),
    };
}

async fn to_show() -> Result<(), Box<dyn Error>> {
    let t = Task::new(Path::new("."))
        .get_cids()
        .await
        .map_err(|e| format!("{e:?}"))?;
    println!("索引 \t cid \t 专辑名");
    t.iter()
        .enumerate()
        .for_each(|(index, (cid, name))| println!("{index} \t {cid} \t {name}"));
    Ok(())
}
