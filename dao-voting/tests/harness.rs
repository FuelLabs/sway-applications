use fuels::{prelude::*, tx::ContractId};
use fuels_abigen_macro::abigen;

// Load abi from json
abigen!(DaoVoting, "out/debug/dao-voting-abi.json");

async fn get_contract_instance() -> (DaoVoting, ContractId) {
    // Launch a local network and deploy the contract
    let wallet = launch_provider_and_get_single_wallet().await;

    let dao_voting_id = Contract::deploy("./out/debug/dao-voting.bin", &wallet, TxParameters::default())
        .await
        .unwrap();

    let dao_voting = DaoVoting::new(dao_voting_id.to_string(), wallet);

    (dao_voting, dao_voting_id)
}

#[tokio::test]
async fn can_get_contract_id() {
    let (_instance, _id) = get_contract_instance().await;
}
