use serde::Deserialize;
use std::io::BufReader;
use std::collections::HashMap;
use std::error::Error;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct List {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub platforms: Option<HashMap<String, Option<String>>>,
}

impl List {
    pub async fn fetch(inc_platforms: bool) -> Result<String, reqwest::Error> {
        let mut link = "https://api.coingecko.com/api/v3/coins/list?include_platform=".to_owned();
        link.push_str(
            if inc_platforms {
                "true"
            } else {
                "false"
            });

        let body = reqwest::get(link)
            .await?
            .text()
            .await?;
        Ok(body)
    }

    pub fn from_str(s: &str) -> Result<Vec<List>, Box<dyn Error>> {
        let reader = BufReader::new(s.as_bytes());
        let u: Vec<List> = serde_json::from_reader(reader)?;
        Ok(u)
    }
}
