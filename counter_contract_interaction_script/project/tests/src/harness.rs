use fuels::{prelude::*, accounts::wallet::WalletUnlocked};

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

    let storage_configuration = StorageConfiguration::load_from("../contract/out/debug/counter_contract-storage_slots.json");
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
async fn test_increment() {
    let (instance, _wallet) = setup().await;

    let result = instance.methods().increment().call().await.unwrap().value;

    assert!(result == 1);
}

#[tokio::test]
async fn test_script() {
    let (instance, wallet) = setup().await;

    let bin_path = "../script/out/debug/interaction_script.bin";
    let script_instance = InteractionScript::new(wallet, bin_path);

    let _result = script_instance.main(instance.id()).set_contracts(&[&instance]).call().await.unwrap();
}