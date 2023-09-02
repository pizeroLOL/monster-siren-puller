use std::env::{self};

use monster_siren_puller::{
    self,
    download::{download_all, download_sync},
    repair,
};

use crate::cmds::{album::AlbumCmd, root::Cmd};

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
        "all" => {
            let re = download_all().await;
            Cmd::try_or_eprintln(re)
        }
        "sync" => {
            let re = download_sync().await;
            Cmd::try_or_eprintln(re)
        }
        "repair" => {
            let re = repair();
            Cmd::try_or_eprintln(re)
        }
        "show" => {
            let re = Cmd::to_show().await;
            Cmd::try_or_eprintln(re)
        }
        "album" => {
            let re = AlbumCmd::main(env).await;
            Cmd::try_or_eprintln(re)
        }
        &_ => Cmd::help(),
    }
}
