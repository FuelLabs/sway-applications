use fuels::{prelude::*, tx::ContractId};
use fuels_abigen_macro::abigen;

// Load abi from json
abigen!(DaoVoting, "out/debug/dao-voting-abi.json");
abigen!(
    GovToken,
    "tests/artifacts/gov_token/out/debug/gov_token-abi.json"
);

async fn setup() -> (DaoVoting, ContractId, ContractId) {
    // Launch a local network and deploy the contract
    let wallet = launch_provider_and_get_single_wallet().await;

    let dao_voting_id = Contract::deploy(
        "./out/debug/dao-voting.bin",
        &wallet,
        TxParameters::default(),
    )
    .await
    .unwrap();

    let gov_token_id = Contract::deploy(
        "./tests/artifacts/gov_token/out/debug/gov_token-abi.json",
        &wallet,
        TxParameters::default(),
    )
    .await
    .unwrap();

    let dao_voting = DaoVoting::new(dao_voting_id.to_string(), wallet);

    (dao_voting, dao_voting_id, gov_token_id)
}

async fn initialize() -> bool {
    let (dao_voting, dao_voting_id, gov_token_id) = setup().await;
    dao_voting
        .constructor(gov_token_id, 10, 10, [0; 32])
        .call()
        .await
        .unwrap()
        .value
}

#[tokio::test]
async fn initializes() {
    assert!(initialize().await);
}

#[tokio::test]
#[should_panic]
async fn panics_when_reinitialized() {
    let (dao_voting, dao_voting_id, gov_token_id) = setup().await;
    dao_voting
        .constructor(gov_token_id, 10, 10, [0; 32])
        .call()
        .await
        .unwrap()
        .value;
    dao_voting
        .constructor(gov_token_id, 10, 10, [0; 32])
        .call()
        .await
        .unwrap()
        .value;
}

#[tokio::test]
#[should_panic]
async fn panics_with_incorrect_voting_period() {
    let (dao_voting, dao_voting_id, gov_token_id) = setup().await;
    dao_voting
        .constructor(gov_token_id, 0, 10, [0; 32])
        .call()
        .await
        .unwrap()
        .value;
}

#[tokio::test]
#[should_panic]
async fn panics_with_incorrect_approval_percentage() {
    let (dao_voting, dao_voting_id, gov_token_id) = setup().await;
    dao_voting
        .constructor(gov_token_id, 10, 0, [0; 32])
        .call()
        .await
        .unwrap()
        .value;
}
