mod cmds;

use crate::cmds::album::AlbumCmd;
use ::clap::Parser;
use cmds::{
    clap::{AlbumCommand, Cli, Commands},
    try_or_eprintln::OkOrEPrintln,
};
use monster_siren_puller::{
    self,
    download_interface::{download_all, download_sync, download_top, get_cids},
    repair,
};
use std::{error::Error, path::Path};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let dir = cli.dir.unwrap_or(Path::new("./siren/").to_path_buf());
    let dir = dir.as_path();

    match cli.command {
        Commands::Top { index } => download_top(dir, index).await.ok_or_eprintln(),
        Commands::All => download_all(dir).await.ok_or_eprintln(),
        Commands::Sync => download_sync(dir).await.ok_or_eprintln(),
        Commands::Repair => repair(dir).ok_or_eprintln(),
        Commands::Show => to_show().await.ok_or_eprintln(),
        Commands::Album { cid, command } => album(dir, cid, command).await,
    }
}

async fn album(dir: &Path, cid: usize, cmd: AlbumCommand) {
    match cmd {
        AlbumCommand::About => AlbumCmd::about(cid).await.ok_or_eprintln(),
        AlbumCommand::Show => AlbumCmd::show(cid).await.ok_or_eprintln(),
        AlbumCommand::Get => AlbumCmd::get(dir, cid).await.ok_or_eprintln(),
    };
}

async fn to_show() -> Result<(), Box<dyn Error>> {
    let t = get_cids().await?;
    println!("索引 \t cid \t 专辑名");
    t.iter()
        .enumerate()
        .for_each(|(index, (cid, name))| println!("{index} \t {cid} \t {name}"));
    Ok(())
}
