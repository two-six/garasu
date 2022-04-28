use std::error::Error;
use std::io::BufReader;
use std::collections::HashMap;

pub struct Parameters {
    pub ids: Vec<String>,
    pub vs_currencies: Vec<String>,
    pub include_market_cap: bool,
    pub include_24hr_vol: bool,
    pub include_24hr_change: bool,
    pub include_last_updated_at: bool,
}

fn bool_as_str<'a>(b: bool) -> &'a str {
    match b {
        true => "true",
        _    => "false"
    }
}

impl Parameters {
    pub fn new(
        ids                     : Vec<String>,
        vs_currencies           : Vec<String>,
        include_market_cap      : bool,
        include_24hr_vol        : bool,
        include_24hr_change     : bool,
        include_last_updated_at : bool,
    ) -> Self {
        Self {
            ids,
            vs_currencies,
            include_market_cap,
            include_24hr_vol,
            include_24hr_change,
            include_last_updated_at
        }
    }

    pub fn to_string(&self) -> String {
        let mut link                      = String::from("https://api.coingecko.com/api/v3/simple/price");
        let mut ids_s                     = String::from("?ids=");
        let mut vs_currencies_s           = String::from("&vs_currencies=");
        let mut include_market_cap_s      = String::from("&include_market_cap=");
        let mut include_24hr_vol_s        = String::from("&include_24hr_vol=");
        let mut include_24hr_change_s     = String::from("&include_24hr_change=");
        let mut include_last_updated_at_s = String::from("&include_last_updated_at=");

        ids_s.push_str(&self.ids.join("%2C"));
        vs_currencies_s.push_str(&self.vs_currencies.join("%2C"));
        include_market_cap_s.push_str(bool_as_str(self.include_market_cap));
        include_24hr_vol_s.push_str(bool_as_str(self.include_24hr_vol));
        include_24hr_change_s.push_str(bool_as_str(self.include_24hr_change));
        include_last_updated_at_s.push_str(bool_as_str(self.include_last_updated_at));

        link.push_str(&ids_s);
        link.push_str(&vs_currencies_s);
        link.push_str(&include_market_cap_s);
        link.push_str(&include_24hr_vol_s);
        link.push_str(&include_24hr_change_s);
        link.push_str(&include_last_updated_at_s);

        link
    }

}

pub async fn fetch(p: &Parameters) -> Result<String, reqwest::Error> {
    let body = reqwest::get(p.to_string())
        .await?
        .text()
        .await?;

    Ok(body)
}

pub fn from_str(s: &str) -> Result<HashMap<String, HashMap<String, f64>>, Box<dyn(Error)>> {
    let reader = BufReader::new(s.as_bytes());
    let u = serde_json::from_reader(reader)?;
    Ok(u)
}
