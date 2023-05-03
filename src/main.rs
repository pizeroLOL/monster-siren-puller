use monster_siren_puller::download::download_all;

#[tokio::main]
async fn main() {
    match download_all().await {
        Ok(t) => t,
        Err(e) => eprintln!("{}",e),
    };
}
