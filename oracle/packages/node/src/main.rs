use fuels::client::FuelClient;
use fuels::prelude::{Bech32ContractId, ContractId, Provider, WalletUnlocked};
use fuels::signers::fuel_crypto::SecretKey;
use node::{spawn_oracle_updater_job, NetworkPriceProvider};
use std::env;
use std::str::FromStr;
use std::time::Duration;
use utils::Oracle;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let api_url_str = env::var("API_URL").expect("API_URL must be set.");
    let api_url = api_url_str
        .parse()
        .unwrap_or_else(|_| panic!("API_URL: '{api_url_str}' is not a valid URL!"));
    let id = Bech32ContractId::from(
        ContractId::from_str(
            &env::var("ORACLE_CONTRACT_ID").expect("ORACLE_CONTRACT_ID must be set."),
        )
        .unwrap(),
    );
    let provider = Provider::new(
        FuelClient::new(env::var("FUEL_PROVIDER_URL").expect("FUEL_PROVIDER_URL must be set."))
            .unwrap(),
    );
    let key = SecretKey::from_str(&env::var("WALLET_SECRET").expect("WALLET_SECRET must be set."))
        .unwrap();
    let unlocked = WalletUnlocked::new_from_private_key(key, Some(provider));
    let oracle = Oracle::new(id.to_string(), unlocked);
    let (handle, receipts_receiver) = spawn_oracle_updater_job(
        oracle,
        Duration::from_secs(10),
        NetworkPriceProvider::new(client, api_url),
    );
}
