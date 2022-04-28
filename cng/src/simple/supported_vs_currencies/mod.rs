use std::error::Error;
use std::io::BufReader;

pub async fn fetch() -> Result<String, reqwest::Error> {
    let body = reqwest::get("https://api.coingecko.com/api/v3/simple/supported_vs_currencies")
        .await?
        .text()
        .await?;

    Ok(body)
}

pub fn from_string(s: &String) -> Result<Vec<String>, Box<dyn(Error)>> {
    let reader = BufReader::new(s.as_bytes());
    let u: Vec<String> = serde_json::from_reader(reader)?;
    Ok(u)
}
