use fuels::{contract::contract::CallResponse, prelude::*};

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
    pub const AMM_CONTRACT_BINARY_PATH: &str = "out/debug/AMM.bin";
    pub const AMM_CONTRACT_STORAGE_PATH: &str = "out/debug/AMM-storage_slots.json";
    pub const EXCHANGE_CONTRACT_BINARY_PATH: &str = "../exchange/out/debug/exchange.bin";
    pub const MALICIOUS_EXCHANGE_CONTRACT_BINARY_PATH: &str =
        "../exchange/tests/artifacts/malicious_implementation/out/debug/malicious_implementation.bin";
}

pub mod test_helpers {
    use super::*;

    use amm_abi_calls::initialize;
    use exchange_abi_calls::constructor;
    use paths::{
        AMM_CONTRACT_BINARY_PATH, AMM_CONTRACT_STORAGE_PATH, EXCHANGE_CONTRACT_BINARY_PATH,
        MALICIOUS_EXCHANGE_CONTRACT_BINARY_PATH,
    };

    pub async fn initialize_amm_contract(wallet: &WalletUnlocked, amm_instance: &AMM) {
        let exchange_contract_id = Contract::deploy(
            EXCHANGE_CONTRACT_BINARY_PATH,
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
        malicious: Option<bool>,
        index_for_salt: Option<u8>,
    ) -> ContractId {
        let salt = [index_for_salt.unwrap_or(0u8); 32];

        let exchange_contract_id = Contract::deploy_with_parameters(
            if !malicious.unwrap_or(false) {
                EXCHANGE_CONTRACT_BINARY_PATH
            } else {
                MALICIOUS_EXCHANGE_CONTRACT_BINARY_PATH
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

    pub async fn setup_and_initialize() -> (WalletUnlocked, AMM, Vec<(ContractId, ContractId)>) {
        let (wallet, amm_instance, asset_pairs) = setup().await;

        initialize_amm_contract(&wallet, &amm_instance).await;

        (wallet, amm_instance, asset_pairs)
    }

    pub async fn setup() -> (WalletUnlocked, AMM, Vec<(ContractId, ContractId)>) {
        // setup wallet and provider
        let mut wallet = WalletUnlocked::new_random(None);
        let num_assets = 3;
        let coins_per_asset = 1;
        let amount_per_coin = 1_000_000;
        let (coins, asset_ids) = setup_multiple_assets_coins(
            wallet.address(),
            num_assets,
            coins_per_asset,
            amount_per_coin,
        );
        let (provider, _socket_addr) = setup_test_provider(coins.clone(), vec![], None).await;
        wallet.set_provider(provider);

        // setup AMM contract
        let amm_contract_id = Contract::deploy(
            AMM_CONTRACT_BINARY_PATH,
            &wallet,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(AMM_CONTRACT_STORAGE_PATH.to_string())),
        )
        .await
        .unwrap();
        let amm_instance = AMM::new(amm_contract_id.to_string(), wallet.clone());

        // setup two asset pairs that will be used in tests
        let asset_pairs = vec![
            (
                ContractId::new(*asset_ids[0]),
                ContractId::new(*asset_ids[1]),
            ),
            (
                ContractId::new(*asset_ids[1]),
                ContractId::new(*asset_ids[2]),
            ),
        ];

        (wallet, amm_instance, asset_pairs)
    }
}