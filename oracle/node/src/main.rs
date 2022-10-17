use fuels::{signers::{WalletUnlocked, fuel_crypto::SecretKey}, prelude::Provider, client::FuelClient};
use reqwest;
use dotenv::dotenv;
use serde::Deserialize;
use tokio::time::{ self, Duration };
use std::env;
use utils::{
    abi_calls::set_price,
    Oracle,
};
use std::str::FromStr;

#[derive(Deserialize)]
struct USDPrice {
    USD: f64,
}

#[tokio::main]
async fn main() {
    let mut env_path = env::current_dir().unwrap();
    env_path.push(std::path::Path::new("node"));
    env::set_current_dir(env_path).unwrap();
    dotenv().ok();
    let api_url = std::env::var("API_URL").expect("API_URL must be set.");
    let oracle_id = env::var("ORACLE_CONTRACT_ID").expect("ORACLE_CONTRACT_ID must be set.");
    let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set.");
    let provider = env::var("FUEL_PROVIDER_URL").expect("FUEL_PROVIDER_URL must be set.");
    let wallet = WalletUnlocked::new_from_private_key(SecretKey::from_str(&wallet_secret).unwrap(), Option::Some(Provider::new(FuelClient::new(provider).unwrap())));
    let client = reqwest::Client::new();
    let mut interval = time::interval(Duration::from_millis(10000));
    let oracle = Oracle::new(oracle_id, wallet);
    interval.tick().await;
    let mut i = 0;
    while i < 2 {
        let response = client.get(api_url.clone()).send().await.unwrap().json::<USDPrice>().await.unwrap();
        // TODO avoid hardcoding the 1e9 decimal precision
        let usd_price = (response.USD * 1e9) as u64;
        set_price(&oracle, usd_price);
        println!("{:?}", usd_price);
        i += 1;
        interval.tick().await;
    }
}
