// use monster_siren_puller::MonsterSiren;
use serde::{Deserialize, Serialize};
static API: &str = "https://monster-siren.hypergryph.com/api/";
static USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/112.0";

#[derive(Serialize, Deserialize, Debug)]

struct AlbumsRequest {
    code: isize,
    msg: String,
    data: Vec<GetAlums>,
}
#[warn(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct GetAlums {
    cid: String,
    name: String,
    coverUrl:String,
    artistes: Vec<String>,
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .unwrap();

    let t = client
        .get(API.to_owned() + "albums/")
        .send()
        .await
        .unwrap()
        .json::<AlbumsRequest>()
        .await
        .unwrap();
    println!("t:{:?}", t.data)
}
