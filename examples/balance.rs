use race_bundlr_sdk::{bundlr::get_balance, currency::CurrencyType};
use reqwest::Url;

#[tokio::main]
async fn main() {
    let url = Url::parse("https://node1.bundlr.network").unwrap();
    let currency = CurrencyType::Solana;
    let address = "A7PPQhbTrKZTF8jzYPAVbtHwpQ2aSYTiznawRe4dCEUe";
    let res = get_balance(&url, currency, address, &reqwest::Client::new()).await;
    match res {
        Ok(ok) => println!("[ok] {}", ok),
        Err(err) => println!("[err] {}", err),
    }
}
