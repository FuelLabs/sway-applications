use fuels::prelude::*;

// Load abi from json
abigen!(MyContract, "dutch-auction/out/debug/dutch-auction-abi.json");

pub async fn get_contract_instance() -> MyContract {
    // Launch a local network and deploy the contract
    let wallet = launch_provider_and_get_wallet().await;

    let id = Contract::deploy(
        "./out/debug/dutch-auction.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "./out/debug/dutch-auction-storage_slots.json"
                .to_string(),
        )),
    )
    .await
    .unwrap();

    MyContractBuilder::new(id.to_string(), wallet.clone()).build()
}

pub async fn active_auctions_of_author(instance: &MyContract, author: Identity) -> Vec<u64> {
    instance.active_auctions_of_author(author).call().await.unwrap().value
}