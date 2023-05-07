fn main() {
    let mut coin = String::new();
    let  result = std::io::stdin().read_line(&mut coin);
    match result { 
        Ok(_) => { get_price(coin) }
        Err(error) => println!("{}", error)
    }
}
fn get_price(coin) {
    let url = format!("https://api.coinmarketcap.com/v1/ticker/{}", coin);
    let resp = reqwest::get(&url).unwrap();
    let mut body = String::new();
    resp.read_to_string(&mut body).unwrap();
    let json: Value = serde_json::from_str(&body).unwrap();
    let price = json[0]["price_usd"].as_str().unwrap();
    println!("The price of {} is {}", coin, price);
}
