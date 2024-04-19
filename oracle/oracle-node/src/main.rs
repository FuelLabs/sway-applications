use dotenv::dotenv;
use fuels::{
    accounts::{fuel_crypto::SecretKey, wallet::WalletUnlocked},
    prelude::{Bech32ContractId, ContractId, Provider},
    tx::ConsensusParameters,
};
use oracle_node::{spawn_oracle_updater_job, NetworkPriceProvider};
use reqwest::Url;
use std::{env, str::FromStr, time::Duration};
use utils::Oracle;

#[tokio::main]
async fn main() {
    let (oracle, client, api_url) = setup();
    let (handle, _receipts_receiver) = spawn_oracle_updater_job(
        oracle,
        Duration::from_secs(10),
        NetworkPriceProvider::new(client, api_url),
    );
    handle.await.unwrap();
}

/// Initialize and return objects for use in main
fn setup() -> (Oracle<WalletUnlocked>, reqwest::Client, Url) {
    let root_env_path = env::current_dir().unwrap();
    let env_path = root_env_path.join("project").join("oracle-node");
    env::set_current_dir(env_path).unwrap();
    dotenv().ok();

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
        env::var("FUEL_PROVIDER_URL").expect("FUEL_PROVIDER_URL must be set."),
        ConsensusParameters::default(),
    )
    .unwrap();

    let key = SecretKey::from_str(&env::var("WALLET_SECRET").expect("WALLET_SECRET must be set."))
        .unwrap();

    let unlocked = WalletUnlocked::new_from_private_key(key, Some(provider));
    let oracle = Oracle::new(id, unlocked);

    (oracle, client, api_url)
}
