use fuels::{contract::contract::CallResponse, prelude::*};
use rand::prelude::Rng;
use std::str::FromStr;

abigen!(AMM, "out/debug/amm-abi.json");
abigen!(Exchange, "../exchange/out/debug/exchange-abi.json");

pub mod exchange_abi_calls {
    use super::*;

    pub async fn constructor(
        contract: &Exchange,
        pair: (ContractId, ContractId),
    ) -> CallResponse<()> {
        let receipt = contract.methods().constructor(pair).call().await;
        dbg!(&receipt);
        receipt.unwrap()
    }
}

pub mod amm_abi_calls {
    use super::*;

    pub async fn add_pool(
        contract: &AMM,
        asset_pair: (ContractId, ContractId),
        pool: ContractId,
    ) -> CallResponse<()> {
        contract
            .methods()
            .add_pool(asset_pair, pool)
            .set_contracts(&[pool.into()])
            .call()
            .await
            .unwrap()
    }

    pub async fn pool(contract: &AMM, asset_pair: (ContractId, ContractId)) -> Option<ContractId> {
        contract
            .methods()
            .pool(asset_pair)
            .call()
            .await
            .unwrap()
            .value
    }
}

pub mod test_helpers {
    use super::*;

    use exchange_abi_calls::constructor;

    pub async fn deploy_exchange_contract(
        wallet: &WalletUnlocked,
        asset_pair: (ContractId, ContractId),
    ) -> ContractId {
        let mut rng = rand::thread_rng();
        let salt: [u8; 32] = rng.gen();

        let exchange_contract_id = Contract::deploy_with_parameters(
            "../exchange/out/debug/exchange.bin",
            &wallet,
            TxParameters::default(),
            StorageConfiguration::default(),
            Salt::from(salt),
        )
        .await
        .unwrap();

        let exchange_instance = Exchange::new(exchange_contract_id.to_string(), wallet.clone());

        constructor(&exchange_instance, asset_pair).await;

        ContractId::new(*exchange_contract_id.hash())
    }

    pub async fn setup() -> (WalletUnlocked, AMM, Vec<ContractId>) {
        let wallet = launch_provider_and_get_wallet().await;

        let amm_contract_id = Contract::deploy(
            "out/debug/amm.bin",
            &wallet,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(
                "out/debug/amm-storage_slots.json".to_string(),
            )),
        )
        .await
        .unwrap();
        let amm_instance = AMM::new(amm_contract_id.to_string(), wallet.clone());

        let base_asset_id = ContractId::from_str(
            "0x0000000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap();
        let asset_id_1 = ContractId::from_str(
            "0x562a05877b940cc69d7a9a71000a0cfdd79e93f783f198de893165278712a480",
        )
        .unwrap();
        let asset_id_2 = ContractId::from_str(
            "0x716c345b96f3c17234c73881c40df43d3d492b902a01a062c12e92eeae0284e9",
        )
        .unwrap();
        (
            wallet,
            amm_instance,
            vec![base_asset_id, asset_id_1, asset_id_2],
        )
    }
}
