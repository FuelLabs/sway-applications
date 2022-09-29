use fuels::{prelude::*, tx::ContractId};

// Load abi from json
abigen!(MyContract, "out/debug/name-registry-abi.json");

pub async fn get_contract_instance() -> (MyContract, ContractId) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(1),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
    )
    .await;
    let wallet = wallets.pop().unwrap();

    let id = Contract::deploy(
        "./out/debug/name-registry.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "./out/debug/name-registry-storage_slots.json".to_string(),
        )),
    )
    .await
    .unwrap();

    let instance = MyContractBuilder::new(id.to_string(), wallet).build();

    (instance, id.into())
}

pub async fn extend(instance: &MyContract, name: String, duration: u64) {
    instance.extend(SizedAsciiString::<8>::new(name).unwrap(), duration).call_params(CallParameters {
        amount: duration/100,
        asset_id: AssetId::BASE,
        gas_forwarded: Some(1000000),
    }).call().await.unwrap();
}

pub async fn register(instance: &MyContract, name: String, duration: u64) {
    instance.register(SizedAsciiString::<8>::new(name).unwrap(), duration).call_params(CallParameters {
        amount: duration/100,
        asset_id: AssetId::BASE,
        gas_forwarded: Some(1000000),
    }).call().await.unwrap();
}

pub async fn set_identity(instance: &MyContract, name: String, identity: Identity) {
    instance.set_identity(SizedAsciiString::<8>::new(name).unwrap(), identity).call().await.unwrap();
}

pub async fn set_owner(instance: &MyContract, name: String, new_owner: Identity) {
    instance.set_owner(SizedAsciiString::<8>::new(name).unwrap(), new_owner).call().await.unwrap();
}