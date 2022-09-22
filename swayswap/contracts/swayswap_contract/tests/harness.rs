use fuel_tx::{ContractId};
use fuels::prelude::*;
use fuels_abigen_macro::abigen;
use std::str::FromStr;

///////////////////////////////
// Load the SwaySwap Contract abi
///////////////////////////////
abigen!(TestSwaySwap, "out/debug/swayswap_contract-abi.json");

#[tokio::test]
async fn swayswap() {
    // Provider and Wallet
    let wallet = launch_provider_and_get_wallet().await;

    // Get the contract ID and a handle to it
    let swayswap_contract_id = Contract::deploy(
        "out/debug/swayswap_contract.bin",
        &wallet,
        TxParameters::default(),
    )
    .await
    .unwrap();
    let swayswap_instance = TestSwaySwap::new(
        swayswap_contract_id.to_string(),
        wallet.clone(),
    );

    // Create fake contract ids
    let token_id = ContractId::from_str("0x562a05877b940cc69d7a9a71000a0cfdd79e93f783f198de893165278712a480").unwrap();
    let swayswap_id = ContractId::from_str("0x014587212741268ad0b1bc727efce9711dbde69c484a9db38bd83bb1b3017c05").unwrap();
    let token_id_2 = ContractId::from_str("0x716c345b96f3c17234c73881c40df43d3d492b902a01a062c12e92eeae0284e9").unwrap();
    let swayswap_id_2 = ContractId::from_str("0x1c74b79b2c430e13380f51258434752ef661e6ebbb9d4970688424e0a63b8070").unwrap();

    let _result = swayswap_instance
        .add_exchange_contract(
            token_id,
            swayswap_id
        )
        .call()
        .await;
    let _result = swayswap_instance
        .add_exchange_contract(
            token_id_2,
            swayswap_id_2
        )
        .call()
        .await;
    
    let result = swayswap_instance
        .get_exchange_contract(token_id)
        .call()
        .await
        .unwrap();
    assert_eq!(result.value, swayswap_id);
    
    let result = swayswap_instance
        .get_exchange_contract(token_id_2)
        .call()
        .await
        .unwrap();
    assert_eq!(result.value, swayswap_id_2);
}
