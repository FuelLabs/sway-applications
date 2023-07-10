use core::fmt::Debug;
use fuels::{
    accounts::Account as FuelAccount,
    prelude::{
        abigen, launch_custom_provider_and_get_wallets, Address, Contract, LoadConfiguration,
        StorageConfiguration, TxParameters, WalletUnlocked, WalletsConfig,
    },
    programs::{call_response::FuelCallResponse, contract::ContractCallHandler},
    types::{traits::Tokenizable, Identity, SizedAsciiString},
};

abigen!(Contract(
    name = "NameRegistry",
    abi = "./contracts/registry-contract/out/debug/registry-contract-abi.json"
));

pub(crate) const REGISTER_DURATION: u64 = 10000;
pub(crate) const EXTEND_DURATION: u64 = 2500;

const CONTRACT_BINARY_PATH: &str = "./out/debug/registry-contract.bin";
const CONTRACT_STORAGE_PATH: &str = "./out/debug/registry-contract-storage_slots.json";

pub(crate) struct Account {
    pub(crate) wallet: WalletUnlocked,
    pub(crate) name: String,
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

pub(crate) async fn setup() -> (NameRegistry<WalletUnlocked>, Account, WalletUnlocked) {
    let number_of_wallets = 2;
    let coins_per_wallet = 1;
    let amount_per_coin = 1_000_000_000;

    let config = WalletsConfig::new(
        Some(number_of_wallets),
        Some(coins_per_wallet),
        Some(amount_per_coin),
    );

    let mut wallets = launch_custom_provider_and_get_wallets(config, None, None).await;

    let wallet = wallets.pop().unwrap();
    let wallet2 = wallets.pop().unwrap();

    let storage_configuration = StorageConfiguration::load_from(CONTRACT_STORAGE_PATH);
    let configuration =
        LoadConfiguration::default().set_storage_configuration(storage_configuration.unwrap());

    let id = Contract::load_from(CONTRACT_BINARY_PATH, configuration)
        .unwrap()
        .deploy(&wallet, TxParameters::default())
        .await
        .unwrap();

    let instance = NameRegistry::new(id, wallet.clone());

    (instance, Account::new(wallet), wallet2)
}

pub(crate) fn string_to_ascii(name: &String) -> SizedAsciiString<8> {
    SizedAsciiString::<8>::new(name.to_owned()).unwrap()
}

pub(crate) async fn get_timestamp_and_call<T, D>(
    handler: ContractCallHandler<T, D>,
) -> (FuelCallResponse<D>, u64)
where
    T: FuelAccount,
    D: Tokenizable + Debug,
{
    let call_response = handler.call().await.unwrap();

    let time = 5;

    (call_response, time)
}

// TODO: Refactor the above function with the something like the example below,
// once `fuels` >= 0.42.0 is available.
// This will allow testing on currently ignored tests.
/*
pub(crate) async fn get_timestamp_and_call<T, D>(
    handler: ContractCallHandler<T, D>,
) -> (FuelCallResponse<D>, u64)
where
    T: FuelAccount,
    D: Tokenizable + Debug,
{
    let call_response = handler.call().await.unwrap();
    let tx_id = call_response.tx_id;

    let provider = handler.account.try_provider().unwrap();
    let tx_response = provider
        .get_transaction_by_id(&tx_id)
        .await
        .unwrap()
        .unwrap();

    let time = tx_response.time.unwrap().timestamp().try_into().unwrap();

    (call_response, time)
}
*/
