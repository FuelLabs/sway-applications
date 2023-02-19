use core::fmt::Debug;
use fuels::{
    core::traits::Tokenizable,
    prelude::{
        abigen, launch_custom_provider_and_get_wallets, Address, Contract, StorageConfiguration,
        TxParameters, WalletUnlocked, WalletsConfig,
    },
    programs::{call_response::FuelCallResponse, contract::ContractCallHandler},
    types::{Identity, SizedAsciiString},
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

pub(crate) async fn setup() -> (NameRegistry, Account, WalletUnlocked) {
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

    let id = Contract::deploy(
        CONTRACT_BINARY_PATH,
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(CONTRACT_STORAGE_PATH.to_string())),
    )
    .await
    .unwrap();

    let instance = NameRegistry::new(id, wallet.clone());

    (instance, Account::new(wallet), wallet2)
}

pub(crate) fn string_to_ascii(name: &String) -> SizedAsciiString<8> {
    SizedAsciiString::<8>::new(name.to_owned()).unwrap()
}

pub(crate) async fn get_timestamp_and_call<T>(
    handler: ContractCallHandler<T>,
) -> (FuelCallResponse<T>, u64)
where
    T: Tokenizable + Debug,
{
    let call_response = handler.call().await.unwrap();

    // TODO: this needs to be updated / reverted when the SDK fixes their breaking changes
    // let script = handler.get_executable_call().await.unwrap();
    // let provider = handler.provider.clone();
    // let tx_id = script.tx.id().to_string();
    // let tx_status = provider
    //     .get_transaction_by_id(&tx_id)
    //     .await
    //     .unwrap()
    //     .unwrap();

    // let time = match tx_status.status {
    //     TransactionStatus::Success() => ( /* get time from here like before */ ),
    //     _ => panic!("tx failed"),
    // }

    let time = 5;

    (call_response, time)
}
