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

struct OracleNode {
    api_url: String,
    client: reqwest::Client,
    duration: time::Duration,
    oracle: Oracle,
    running: bool,
}

impl OracleNode {
    pub fn new(seconds: u64) -> Self {
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
        let duration = time::Duration::from_secs(seconds);
        let oracle = Oracle::new(bech32_oracle_id.to_string(), wallet);

        Self {
            api_url,
            client,
            duration,
            oracle,
            running: true,
        }
    }

    pub async fn run(&self) {
        while self.running {
            let usd_price = self.get_price().await;
            set_price(&self.oracle, usd_price).await;
            sleep(self.duration).await;
        }
    }

    async fn get_price(&self) -> u64 {
        let response = self.client
            .get(&self.api_url)
            .send()
            .await
            .unwrap()
            .json::<USDPrice>()
            .await
            .unwrap();
        // TODO avoid hardcoding the 1e9 decimal precision
        (response.USD * 1e9) as u64
    }
}

#[tokio::main]
async fn main() {
    let oracle_node = OracleNode::new(10);
    oracle_node.run().await;
}   
