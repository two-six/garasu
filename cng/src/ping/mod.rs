pub async fn ping() -> Result<String, reqwest::Error> {
    let body = reqwest::get("https://api.coingecko.com/api/v3/ping")
        .await?
        .text()
        .await?;
    Ok(body)
}
