use serde::Deserialize;

use std::error::Error;
use std::io::BufReader;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Trending {
    coins: Vec<HashMap<String, Item>>
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Item {
    id: String,
    coin_id: i32,
    name: String,
    symbol: String,
    market_cap_rank: i32,
    thumb: String,
    small: String,
    large: String,
    slug: String,
    price_btc: f64,
    score: i32,
}

fn trending(s: &String) -> Result<Trending, Box<dyn Error>> {
    let reader = BufReader::new(s.as_bytes());
    let u: Trending = serde_json::from_reader(reader)?;
    Ok(u)
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let body = reqwest::get("https://api.coingecko.com/api/v3/search/trending")
        .await?
        .text()
        .await?;

    // println!("{:#?}", trending(&body).unwrap());

    for el in trending(&body).unwrap().coins {
        for v in el {
            println!("{:.8} -> ₿ {:.8}", v.1.name, v.1.price_btc);
        }
    }

    Ok(())
}
