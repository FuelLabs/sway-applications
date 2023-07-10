use fuels::{
    accounts::wallet::WalletUnlocked,
    prelude::{abigen, Contract, LoadConfiguration, StorageConfiguration, TxParameters},
    programs::contract::SettableContract,
    test_helpers::{launch_custom_provider_and_get_wallets, WalletsConfig},
};

abigen!(
    Contract(
        name = "CounterContract",
        abi = "./contract/out/debug/counter_contract-abi.json"
    ),
    Script(
        name = "InteractionScript",
        abi = "./script/out/debug/interaction_script-abi.json"
    )
);

pub async fn setup() -> (CounterContract<WalletUnlocked>, WalletUnlocked) {
    let number_of_wallets = 1;
    let coins_per_wallet = 1;
    let amount_per_coin = 1_000_000_000;

    let config = WalletsConfig::new(
        Some(number_of_wallets),
        Some(coins_per_wallet),
        Some(amount_per_coin),
    );

    let mut wallets = launch_custom_provider_and_get_wallets(config, None, None).await;

    let wallet = wallets.pop().unwrap();

    let storage_configuration = StorageConfiguration::load_from(
        "../contract/out/debug/counter_contract-storage_slots.json",
    );
    let configuration =
        LoadConfiguration::default().set_storage_configuration(storage_configuration.unwrap());

    let id = Contract::load_from("../contract/out/debug/counter_contract.bin", configuration)
        .unwrap()
        .deploy(&wallet, TxParameters::default())
        .await
        .unwrap();

    let instance = CounterContract::new(id, wallet.clone());

    (instance, wallet)
}

#[tokio::test]
async fn test_script_clearing_at_end() {
    let (instance, wallet) = setup().await;

    let bin_path = "../script/out/debug/interaction_script.bin";
    let script_instance = InteractionScript::new(wallet, bin_path);

    let result = script_instance
        .main(instance.id(), true)
        .set_contracts(&[&instance])
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(result, 0);
}

#[tokio::test]
async fn test_script_not_clearing_at_end() {
    let (instance, wallet) = setup().await;

    let bin_path = "../script/out/debug/interaction_script.bin";
    let script_instance = InteractionScript::new(wallet, bin_path);

    let result = script_instance
        .main(instance.id(), false)
        .set_contracts(&[&instance])
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(result, 2);
}
