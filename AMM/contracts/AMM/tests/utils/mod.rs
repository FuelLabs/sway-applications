use fuels::{contract::contract::CallResponse, prelude::*};
use std::str::FromStr;

abigen!(AMM, "out/debug/AMM-abi.json");
abigen!(Exchange, "../exchange/out/debug/exchange-abi.json");

pub mod exchange_abi_calls {
    use super::*;

    pub async fn constructor(
        contract: &Exchange,
        pair: (ContractId, ContractId),
    ) -> CallResponse<()> {
        let receipt = contract.methods().constructor(pair).call().await;
        receipt.unwrap()
    }
}

pub mod amm_abi_calls {
    use super::*;

    pub async fn initialize(contract: &AMM, exchange_id: ContractId) -> CallResponse<()> {
        contract
            .methods()
            .initialize(exchange_id)
            .call()
            .await
            .unwrap()
    }

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

pub mod paths {
    pub const AMM_CONTRACT_BINARY_PATH: &str = "out/debug/amm.bin";
    pub const AMM_CONTRACT_STORAGE_PATH: &str = "out/debug/amm-storage_slots.json";
    pub const INVALID_EXCHANGE_CONTRACT_BINARY_PATH: &str =
        "../exchange/tests/artifacts/faulty_implementation/out/debug/faulty_implementation.bin";
    pub const VALID_EXCHANGE_CONTRACT_BINARY_PATH: &str = "../exchange/out/debug/exchange.bin";
}

pub mod test_helpers {
    use super::*;

    use amm_abi_calls::initialize;
    use exchange_abi_calls::constructor;
    use paths::{
        AMM_CONTRACT_BINARY_PATH, AMM_CONTRACT_STORAGE_PATH, INVALID_EXCHANGE_CONTRACT_BINARY_PATH,
        VALID_EXCHANGE_CONTRACT_BINARY_PATH,
    };

    pub async fn initialize_amm_contract(wallet: &WalletUnlocked, amm_instance: &AMM) {
        let exchange_contract_id = Contract::deploy(
            VALID_EXCHANGE_CONTRACT_BINARY_PATH,
            &wallet,
            TxParameters::default(),
            StorageConfiguration::default(),
        )
        .await
        .unwrap();

        initialize(amm_instance, exchange_contract_id.into()).await;
    }

    pub async fn deploy_and_construct_exchange_contract(
        wallet: &WalletUnlocked,
        asset_pair: (ContractId, ContractId),
        valid: Option<bool>,
        index_for_salt: Option<u8>,
    ) -> ContractId {
        let salt = [index_for_salt.unwrap_or(0u8); 32];

        let exchange_contract_id = Contract::deploy_with_parameters(
            if valid.unwrap_or(true) {
                VALID_EXCHANGE_CONTRACT_BINARY_PATH
            } else {
                INVALID_EXCHANGE_CONTRACT_BINARY_PATH
            },
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

    pub async fn setup_and_initialize() -> (WalletUnlocked, AMM, Vec<ContractId>) {
        let (wallet, amm_instance, assets) = setup().await;

        initialize_amm_contract(&wallet, &amm_instance).await;

        (wallet, amm_instance, assets)
    }

    pub async fn setup() -> (WalletUnlocked, AMM, Vec<ContractId>) {
        let wallet = launch_provider_and_get_wallet().await;

        let amm_contract_id = Contract::deploy(
            AMM_CONTRACT_BINARY_PATH,
            &wallet,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(AMM_CONTRACT_STORAGE_PATH.to_string())),
        )
        .await
        .unwrap();
        let amm_instance = AMM::new(amm_contract_id.to_string(), wallet.clone());

        let base_asset_id = ContractId::zeroed();
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
