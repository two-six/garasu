use log::{info, warn};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    env_logger::init();
    let ping = cng::ping::ping();
    info!("Connecting to CoinGecko...");
    if ping.await.is_ok() {
        info!("Fetching data...");
        let body = cng::Trending::fetch().await?;
        for el in cng::Trending::from_str(&body).unwrap().coins {
            for v in el {
                println!("{:.8} -> ₿ {:.8}", v.1.name, v.1.price_btc);
            }
        }

        let p = cng::simple::price::Parameters::new(
            vec!["bitcoin".to_owned(), "ethereum".to_owned()],
            vec!["usd".to_owned(), "eur".to_owned()],
            false,
            false,
            false,
            false
        );
        let s = cng::simple::price::fetch(&p).await?;
        println!("{:#?}", cng::simple::price::from_str(&s));

    } else {
        warn!("Couldn't connect to the server");
    }

    Ok(())
}

// TODO: simple::price access to individual fields separated by cryptocurrency
