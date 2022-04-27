#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let body = cng::Trending::fetch().await.unwrap();

    // println!("{:#?}", trending(&body).unwrap());

    for el in cng::Trending::from_string(&body).unwrap().coins {
        for v in el {
            println!("{:.8} -> ₿ {:.8}", v.1.name, v.1.price_btc);
        }
    }

    Ok(())
}
