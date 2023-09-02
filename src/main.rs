use crate::cmds::{album::AlbumCmd, root::Cmd};
use cmds::try_or_eprintln::TryOrEPrintln;
use monster_siren_puller::{
    self,
    download::{download_all, download_sync},
    repair,
};
use std::env;

mod cmds;

#[tokio::main]
async fn main() {
    let mut env = env::args();
    env.next();
    let Some(t) = env.next() else {
        return Cmd::help();
    };
    match t.as_str() {
        "help" => Cmd::help(),
        "top" => Cmd::top(env).await,
        "all" => download_all().await.try_or_eprintln(),
        "sync" => download_sync().await.try_or_eprintln(),
        "repair" => repair().try_or_eprintln(),
        "show" => Cmd::to_show().await.try_or_eprintln(),
        "album" => AlbumCmd::main(env).await.try_or_eprintln(),
        &_ => Cmd::help(),
    }
}
