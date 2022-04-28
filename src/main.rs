use log::{info, warn};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    env_logger::init();
    let ping = cng::ping::ping();
    info!("Connecting to CoinGecko...");
    if ping.await.is_ok() {
        info!("Fetching data...");
        let body = cng::Trending::fetch().await?;
        for el in cng::Trending::from_string(&body).unwrap().coins {
            for v in el {
                println!("{:.8} -> ₿ {:.8}", v.1.name, v.1.price_btc);
            }
        }
        info!("Fetching coins list...");
        println!("{:#?}", cng::coins::list::List::from_string(&cng::coins::list::List::fetch(false).await?));
    } else {
        warn!("Couldn't connect to the server");
    }

    Ok(())
}
