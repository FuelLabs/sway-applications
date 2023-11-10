use fuels::{
    accounts::wallet::WalletUnlocked,
    prelude::{abigen, Contract, LoadConfiguration, StorageConfiguration, TxParameters},
    programs::contract::SettableContract,
    test_helpers::launch_provider_and_get_wallet,
};

// The following macro will automatically generate some structs for you, to easily interact with contracts and scripts.
abigen!(
    Contract(
        name = "CounterContract",
        abi = "./contracts/counter/out/debug/counter_contract-abi.json"
    ),
    Script(
        name = "InteractionScript",
        abi = "./scripts/interaction_script/out/debug/interaction_script-abi.json"
    )
);

// File path constants
const STORAGE_CONFIGURATION_PATH: &str =
    "../../contracts/counter/out/debug/counter_contract-storage_slots.json";
const CONTRACT_BIN_PATH: &str = "../../contracts/counter/out/debug/counter_contract.bin";
const SCRIPT_BIN_PATH: &str = "../../scripts/interaction_script/out/debug/interaction_script.bin";

// This function will setup the test environment for you. It will return a tuple containing the contract instance and the script instance.
pub async fn setup() -> (
    CounterContract<WalletUnlocked>,
    InteractionScript<WalletUnlocked>,
) {
    // The `launch_provider_and_get_wallet` function will launch a local provider and create a wallet for you.
    let wallet = launch_provider_and_get_wallet().await;

    // The following code will load the storage configuration (default storage values) from the contract and create a configuration object.
    let storage_configuration =
        StorageConfiguration::default().add_slot_overrides_from_file(STORAGE_CONFIGURATION_PATH).unwrap();
    let configuration =
        LoadConfiguration::default().with_storage_configuration(storage_configuration);

    // The following code will deploy the contract and store the returned ContractId in the `id` variable.
    let id = Contract::load_from(CONTRACT_BIN_PATH, configuration)
        .unwrap()
        .deploy(&wallet, TxParameters::default())
        .await
        .unwrap();

    // Creates a contract instance and a script instance. Which allow for easy interaction with the contract and script.
    let contract_instance = CounterContract::new(id, wallet.clone());
    let script_instance = InteractionScript::new(wallet, SCRIPT_BIN_PATH);

    (contract_instance, script_instance)
}

#[tokio::test]
async fn test_script_clearing_at_end() {
    // Call the setup function to deploy the contract and create the contract and script instances.
    let (contract_instance, script_instance) = setup().await;

    // Execute the script with the `clear` parameter set to true.
    let result = script_instance
        .main(contract_instance.id(), true) // Passing the main function parameters defined in the sway script code.
        .with_contracts(&[&contract_instance]) // Defining the contracts that the script will interact with.
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(result, 0);
}

#[tokio::test]
async fn test_script_not_clearing_at_end() {
    // Call the setup function to deploy the contract and create the contract and script instances.
    let (contract_instance, script_instance) = setup().await;

    // Execute the script with the `clear` parameter set to false.
    let result = script_instance
        .main(contract_instance.id(), false) // Passing the main function parameters defined in the sway script code.
        .with_contracts(&[&contract_instance]) // Defining the contracts that the script will interact with.
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(result, 1);
}
