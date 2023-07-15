use std::{env::Args, error::Error};

use monster_siren_puller::download::{download_top, get_cids};

pub struct Cmd();

impl Cmd {
    pub fn help() {
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
    album \t显示专辑相关功能
            "
        )
    }
    pub fn try_or_eprintln(f: Result<(), Box<dyn Error>>) {
        match f {
            Ok(t) => t,
            Err(e) => println!("{}", e),
        };
    }
    pub async fn top(num: Args) {
        let mut num = num;
        let Some(num)  = num.next() else{
            println!("缺少前top 后参数");
            return Self::help()
        };

        if let Ok(num) = num.parse::<usize>() {
            let re = download_top(num).await;
            Self::try_or_eprintln(re);
        } else {
            println!("请输入数字");
            Self::help();
        }
    }
    pub async fn to_show() -> Result<(), Box<dyn Error>> {
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
}
