use fuels::{signers::{WalletUnlocked, fuel_crypto::SecretKey}, prelude::{Provider, Bech32Address, Bech32ContractId}, client::FuelClient, tx::ContractId};
use reqwest;
use dotenv::dotenv;
use serde::Deserialize;
use tokio::time::{ self, Duration, sleep };
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
    let (oracle, client, api_url, duration) = initialize_node();
    let mut i = 0;
    while i < 2 {
        let response = get_price(&client, &api_url).await;
        // TODO avoid hardcoding the 1e9 decimal precision
        let usd_price = (response.USD * 1e9) as u64;
        set_price(&oracle, usd_price).await;
        println!("{:?}", usd_price);
        i += 1;
        sleep(duration).await;
    }
}

fn initialize_node() -> (Oracle, reqwest::Client, String, Duration) {
    let mut env_path = env::current_dir().unwrap();
    env_path.push(std::path::Path::new("node"));
    env::set_current_dir(env_path).unwrap();
    dotenv().ok();
    let api_url = env::var("API_URL").expect("API_URL must be set.");
    let oracle_id_string = env::var("ORACLE_CONTRACT_ID").expect("ORACLE_CONTRACT_ID must be set.");
    let oracle_id = ContractId::from_str(&oracle_id_string).unwrap();
    let bech32_oracle_id = Bech32ContractId::from(oracle_id);
    let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set.");
    let provider = env::var("FUEL_PROVIDER_URL").expect("FUEL_PROVIDER_URL must be set.");
    let wallet = WalletUnlocked::new_from_private_key(
        SecretKey::from_str(&wallet_secret).unwrap(),
        Option::Some(Provider::new(FuelClient::new(provider).unwrap()))
    );
    let client = reqwest::Client::new();
    let duration = time::Duration::from_secs(10);
    let oracle = Oracle::new(bech32_oracle_id.to_string(), wallet);
    (oracle, client, api_url, duration)
}

async fn get_price(client: &reqwest::Client, api_url: &String) -> USDPrice {
    client
        .get(api_url.clone())
        .send()
        .await
        .unwrap()
        .json::<USDPrice>()
        .await
        .unwrap()
}
