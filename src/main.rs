use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct ExchangeRateResponse {
    motd: Option<MessageOfTheDay>,
    success: bool,
    base: String,
    date: String,
    rates: std::collections::HashMap<String, f64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct MessageOfTheDay {
    msg: String,
    url: String,
}

fn main() {
    let url = "https://api.exchangerate.host/latest?base=USD&symbols=MXN";
    let resp = ureq::get(url).call().unwrap();
    let body = resp.into_string().unwrap();

    let exchange_rate: ExchangeRateResponse = serde_json::from_str(&body).unwrap();
     if let Some(mxn_rate) = exchange_rate.rates.get("MXN") {
        println!(" The current exchange rate is: {}", mxn_rate);
    }
}
