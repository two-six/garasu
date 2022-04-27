// #[tokio::main]
// async fn main() -> Result<(), reqwest::Error> {
//     let body = reqwest::get("https://api.coingecko.com/api/v3/search/trending")
//         .await?
//         .text()
//         .await?;

//     // println!("{:#?}", trending(&body).unwrap());

//     for el in cng::trending(&body).unwrap().coins {
//         for v in el {
//             println!("{:.8} -> ₿ {:.8}", v.1.name, v.1.price_btc);
//         }
//     }

//     Ok(())
// }
