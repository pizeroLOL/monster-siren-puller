// use std::error::Error;

// static API: &str = "https://monster-siren.hypergryph.com/api/";
// static USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/112.0";

// pub struct MonsterSiren {}

// struct AlbumsRequest {
//     code: u8,
//     msg: String,
//     data: Vec<GetAlums>,
// }

// struct GetAlums {
//     cid: isize,
//     name: String,
//     artistes: Vec<String>,
// }

// impl MonsterSiren {
//     pub fn new() -> Result<(), Box<dyn Error>> {
//         let client = reqwest::Client::builder().user_agent(USER_AGENT).build()?;
//         // async {};
//         let runtime = tokio::runtime::Builder::new_multi_thread()
//             .enable_all()
//             .build()?;
//         let t = runtime.block_on(t())?;
//         // let t = runtime.block_on(async {
//         // });
//         println!("t: {:?}",t);
//         Ok(())
//     }
// }

// async fn t() -> Result<(), Box<dyn Error>> {
//     let client = reqwest::Client::builder().user_agent(USER_AGENT).build()?;
//     let t = client
//         .get(API.to_owned() + "albums/")
//         .send()
//         .await
//         .unwrap()
//         .json::<std::collections::HashMap<String,String>>()
//         .await
//         .unwrap();
//     println!("{:?}",t);
//     Ok(())
// }
