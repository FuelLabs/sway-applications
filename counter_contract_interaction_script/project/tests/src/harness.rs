use fuels::prelude::*;

abigen!(Contract(
    name = "CounterContract",
    abi = "./contract/out/debug/counter_contract-abi.json"
));



// pub(crate) async fn setup() -> (NameRegistry<WalletUnlocked>, Account, WalletUnlocked) {
//     let number_of_wallets = 2;
//     let coins_per_wallet = 1;
//     let amount_per_coin = 1_000_000_000;

//     let config = WalletsConfig::new(
//         Some(number_of_wallets),
//         Some(coins_per_wallet),
//         Some(amount_per_coin),
//     );

//     let mut wallets = launch_custom_provider_and_get_wallets(config, None, None).await;

//     let wallet = wallets.pop().unwrap();
//     let wallet2 = wallets.pop().unwrap();

//     let storage_configuration = StorageConfiguration::load_from(CONTRACT_STORAGE_PATH);
//     let configuration =
//         LoadConfiguration::default().set_storage_configuration(storage_configuration.unwrap());

//     let id = Contract::load_from(CONTRACT_BINARY_PATH, configuration)
//         .unwrap()
//         .deploy(&wallet, TxParameters::default())
//         .await
//         .unwrap();

//     let instance = NameRegistry::new(id, wallet.clone());

//     (instance, Account::new(wallet), wallet2)
// }