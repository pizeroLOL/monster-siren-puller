use std::env;

use monster_siren_puller::{
    self,
    download::{download_all, download_sync, download_top},
};

fn help() {
    println!(
        "
monster-siren-puller [help|all|sync]
monster-siren-puller [top] <数字>

help  \t获取该页面
top   \t获取前<参数>首
all   \t获取所有歌曲
sync  \t获取未下载专辑
repair\t删除下载了一半的专辑
        "
    )
}

async fn top(num: Option<String>) {
    let Some(num)  = num else{
        println!("缺少前top 后参数");
        return help()
    };

    if let Ok(num) = num.parse::<usize>() {
        match download_top(num).await {
            Ok(t) => t,
            Err(e) => println!("{}", e),
        };
    } else {
        println!("请输入数字");
        help();
    }
}

async fn all() {
    match download_all().await {
        Ok(t) => t,
        Err(e) => println!("{}", e),
    };
}

async fn sync() {
    match download_sync().await {
        Ok(t) => t,
        Err(e) => println!("{}", e),
    };
}

fn repair() {
    match monster_siren_puller::repair() {
        Ok(t) => t,
        Err(e) => println!("{}", e),
    };
}

#[tokio::main]
async fn main() {
    let mut env = env::args();
    env.next();
    let Some(t) = env.next() else {
        return help()
    };
    match t.as_str() {
        "help" => help(),
        "top" => top(env.next()).await,
        "all" => all().await,
        "sync" => sync().await,
        "repair" => repair(),
        &_ => help(),
    }
}
