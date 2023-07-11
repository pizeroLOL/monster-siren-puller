use std::{
    env::{self, Args},
    error::Error,
};

use monster_siren_puller::{
    self,
    download::{download_all, download_sync, download_top, get_cids},
    repair,
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
show  \t展示专辑 cid 和对应名称
        "
    )
}

async fn try_or_eprintln(f: Result<(), Box<dyn Error>>) {
    match f {
        Ok(t) => t,
        Err(e) => println!("{}", e),
    };
}

async fn top(num: Args) {
    let mut num = num;
    let Some(num)  = num.next() else{
        println!("缺少前top 后参数");
        return help()
    };

    if let Ok(num) = num.parse::<usize>() {
        let re = download_top(num).await;
        try_or_eprintln(re).await;
    } else {
        println!("请输入数字");
        help();
    }
}

async fn to_show() -> Result<(), Box<dyn Error>> {
    let t = get_cids().await?;
    let tips = "cid  \t 专辑名\n";
    let t = t
        .iter()
        .map(|(cid, name)| format!("{cid} \t {name}\n"))
        .collect::<String>();
    let t = tips.to_owned() + &t;
    println!("{}", t);
    Ok(())
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
        "top" => top(env).await,
        "all" => {
            let re = download_all().await;
            try_or_eprintln(re).await
        }
        "sync" => {
            let re = download_sync().await;
            try_or_eprintln(re).await
        }
        "repair" => {
            let re = repair();
            try_or_eprintln(re).await
        }
        "show" => {
            let re = to_show().await;
            try_or_eprintln(re).await
        }
        &_ => help(),
    }
}
