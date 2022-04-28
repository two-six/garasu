use serde::Deserialize;

use std::error::Error;
use std::io::BufReader;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

pub mod ping;
pub mod coins;
pub mod simple;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Trending {
    pub coins: Vec<HashMap<String, Item>>
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Item {
    pub id: String,
    pub coin_id: i32,
    pub name: String,
    pub symbol: String,
    pub market_cap_rank: i32,
    pub thumb: String,
    pub small: String,
    pub large: String,
    pub slug: String,
    pub price_btc: f64,
    pub score: i32,
}

impl Trending {
    #[allow(dead_code)]
    pub fn from_str(s: &str) -> Result<Trending, Box<dyn Error>> {
        let reader = BufReader::new(s.as_bytes());
        let u: Trending = serde_json::from_reader(reader)?;
        Ok(u)
    }

    #[allow(dead_code)]
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Trending, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let u: Trending = serde_json::from_reader(reader)?;
        Ok(u)
    }

    #[allow(dead_code)]
    pub async fn fetch() -> Result<String, reqwest::Error> {
        let body = reqwest::get("https://api.coingecko.com/api/v3/search/trending")
            .await?
            .text()
            .await?;
        Ok(body)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_fail() {
        assert_eq!(Trending::from_str(&"fail".to_owned()).is_err(), true);
    }
    #[test]
    fn fn_from_file() {
        let x = Trending::from_file("example.json");
        assert_eq!(x.is_ok(), true);
    }
}
