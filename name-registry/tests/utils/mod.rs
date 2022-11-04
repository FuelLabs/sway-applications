pub mod abi;

use fuels::{prelude::*};

// Load abi from json
abigen!(NameRegistry, "out/debug/name-registry-abi.json");

pub const REGISTER_DURATION: u64 = 10000;
pub const EXTEND_DURATION: u64 = 2500;

pub async fn setup() -> (NameRegistry, Account, WalletUnlocked) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(2),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
    )
    .await;
    let wallet = wallets.pop().unwrap();
    let wallet2 = wallets.pop().unwrap();

    let id = Contract::deploy(
        "./out/debug/name-registry.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "./out/debug/name-registry-storage_slots.json".to_string(),
        )),
    )
    .await
    .unwrap();

    let instance = NameRegistry::new(id.to_string(), wallet.clone());

    (instance, Account::new(wallet), wallet2)
}

pub fn string_to_ascii(name: &String) -> SizedAsciiString<8> {
    SizedAsciiString::<8>::new(name.to_owned()).unwrap()
}

pub struct Account {
    pub wallet: WalletUnlocked,
    pub name: String,
}

impl Account {
    pub fn new(wallet: WalletUnlocked) -> Self {
        Self {
            wallet,
            name: String::from("SwaySway"),
        }
    }

    pub fn identity(&self) -> Identity {
        Identity::Address(Address::from(self.wallet.address()))
    }
}
