use fuels::{prelude::*, tx::ContractId};

// Load abi from json
abigen!(StakingRewards, "out/debug/staking-rewards-abi.json");

async fn setup() -> (StakingRewards, ContractId, LocalWallet) {
    // Launch a local network and deploy the contract
    let wallet = launch_provider_and_get_wallet().await;

    let id = Contract::deploy(
        "./out/debug/staking-rewards.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "./out/debug/staking-rewards-storage_slots.json".to_string(),
        )),
    )
    .await
    .unwrap();

    let instance = StakingRewards::new(id.to_string(), wallet.clone());

    (instance, id, wallet)
}


// For testing, staking and reward token will both be native asset
// Timestamps of each action must be specified. Contract is deployed at t=0

#[tokio::test]
async fn can_stake() {

    let (staking_contract, _id, wallet) = setup().await;
    
    let amount_to_stake = 1000;
    let timestamp = 0;
    let staking_call_params = CallParameters::new(Some(amount_to_stake), None, None);
    let _receipts = staking_contract.stake(timestamp).call_params(staking_call_params).call().await.unwrap();

    // Check total_supply
    let total_supply = staking_contract.total_supply().call().await.unwrap().value;
    assert_eq!(total_supply, amount_to_stake);
    
    // Check balance
    let wallet_identity = stakingrewards_mod::Identity::Address(wallet.address());
    let user_balance = staking_contract.balance_of(wallet_identity).call().await.unwrap().value;
    assert_eq!(user_balance, amount_to_stake)

}
