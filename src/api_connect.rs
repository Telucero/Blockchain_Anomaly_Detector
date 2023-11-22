// src/api_connect.rs

use reqwest;
use std::env;
use dotenv::dotenv;

pub async fn fetch_data() -> Result<String, reqwest::Error> {
    dotenv().ok();

    let secret_key = env::var("BLOCKCHAIN_SECRET_KEY").expect("Secret key not set");
    //let public_key = env::var("BLOCKCHAIN_PUBLIC_KEY").expect("Public key not set");
    let base_url = "https://api.blockchain.com/v3/exchange";
    let endpoint = "/tickers/BTC-USD"; // Replace with your desired endpoint
    let url = format!("{}{}", base_url, endpoint);

    let client = reqwest::Client::new();
    let response = client.get(&url)
                         .header("X-API-Token", secret_key)
                         .send()
                         .await?
                         .text()
                         .await?;

    Ok(response)
}
