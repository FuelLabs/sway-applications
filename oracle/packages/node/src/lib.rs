use fuels::{signers::{WalletUnlocked, fuel_crypto::SecretKey}, prelude::{Provider, Bech32ContractId}, client::FuelClient, tx::{ContractId, Receipt}};
use reqwest;
use dotenv::dotenv;
use serde::Deserialize;
use tokio::time::{ self, sleep };
use std::env;
use utils::{
    abi_calls::set_price,
    Oracle,
};
use std::str::FromStr;
use futures::executor::block_on;

#[derive(Deserialize)]
struct USDPrice {
    USD: f64,
}

pub struct OracleNode {
    api_url: String,
    client: reqwest::Client,
    pub duration: time::Duration,
    pub oracle: Oracle,
    running: bool,
    transmitter: tokio::sync::mpsc::Sender<u64>,
    pub receiver: tokio::sync::mpsc::Receiver<u64>,
}

impl OracleNode {
    pub fn new(seconds: u64) -> Self {
        let env_path = env::current_dir().unwrap();
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
        let (transmitter, receiver) = tokio::sync::mpsc::channel(100);
        Self {
            api_url,
            client,
            duration,
            oracle,
            running: true,
            transmitter,
            receiver,
        }
    }

    pub async fn run(&self) {
        //let transmitter = std::sync::Arc::clone(&self.transmitter);
        //let thread_self = self.clone();
        tokio::task::spawn_blocking(move || loop {
            let usd_price = block_on(self.get_price());
            println!("{usd_price}");
            let send_me = block_on(self.oracle.methods().set_price(usd_price).call())
                .map(|response| response.receipts)
                .map(|receipts: Vec<Receipt>| {
                    receipts
                        .into_iter()
                        .filter(|receipt| {
                            matches!(receipt, Receipt::Log { .. } | Receipt::LogData { .. })
                        })
                        .collect::<Vec<_>>()
                });
            block_on(self.transmitter.send(usd_price)).unwrap();
            block_on(sleep(self.duration));
        });
    }

    pub async fn get_price(&self) -> u64 {
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
