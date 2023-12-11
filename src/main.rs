mod cmds;
use cmds::{
    clap::{AlbumCommand, Cli, Commands},
    try_or_eprintln::OkOrEPrintln,
};
use monster_siren_puller::{
    download3::{config::DLConfig, download_all, download_sync, download_top},
    repair,
    types::{AlbumIndex, Response},
};

use std::error::Error;

use crate::cmds::album::AlbumCmd;
use ::clap::Parser;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let config = TryInto::<DLConfig>::try_into(cli.clone()).unwrap();

    match cli.command {
        Commands::Top { index } => download_top(index, &config).await.ok_or_eprintln(),
        Commands::All => download_all(&config).await.ok_or_eprintln(),
        Commands::Sync => download_sync(&config).await.ok_or_eprintln(),
        Commands::Repair => repair(&config.dir).ok_or_eprintln(),
        Commands::Show => to_show(&config).await.ok_or_eprintln(),
        Commands::Album { cid, command } => album(cid, command, &config).await,
    }
}

async fn album(cid: usize, cmd: AlbumCommand, config: &DLConfig) {
    match cmd {
        AlbumCommand::About => AlbumCmd::about(cid, config)
            .await
            .map_err(|e| e.into())
            .ok_or_eprintln(),
        AlbumCommand::Show => AlbumCmd::show(cid, config)
            .await
            .map_err(|e| e.into())
            .ok_or_eprintln(),
        AlbumCommand::Get => AlbumCmd::get(cid, config).await.ok_or_eprintln(),
    };
}

async fn to_show(config: &DLConfig) -> Result<(), Box<dyn Error>> {
    let url = AlbumIndex::get_url();
    let album_indexes = Response::<Vec<AlbumIndex>>::get(&url, &config.ua, config.timeout).await?;

    println!("索引 \t cid \t 专辑名");
    album_indexes
        .iter()
        .map(|album_index| (album_index.get_cid(), album_index.get_name()))
        .enumerate()
        .for_each(|(index, (cid, name))| println!("{index} \t {cid} \t {name}"));
    Ok(())
}
