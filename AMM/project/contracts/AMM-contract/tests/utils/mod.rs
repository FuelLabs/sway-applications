use fuels::{contract::contract::CallResponse, prelude::*, tx::Contract as TxContract};

abigen!(
    AMM,
    "./project/contracts/AMM-contract/out/debug/AMM-contract-abi.json"
);
abigen!(
    Exchange,
    "./project/contracts/exchange-contract/out/debug/exchange-contract-abi.json"
);

pub struct ExchangeContract {
    pub contract_id: ContractId,
    pub bytecode_root: ContractId,
}

pub mod paths {
    pub const AMM_CONTRACT_BINARY_PATH: &str = "./out/debug/AMM-contract.bin";
    pub const AMM_CONTRACT_STORAGE_PATH: &str = "./out/debug/AMM-contract-storage_slots.json";
    pub const EXCHANGE_CONTRACT_BINARY_PATH: &str =
        "../exchange-contract/out/debug/exchange-contract.bin";
    pub const MALICIOUS_EXCHANGE_CONTRACT_BINARY_PATH: &str =
        "../exchange-contract/tests/artifacts/malicious-implementation/out/debug/malicious-implementation.bin";
}

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

    pub async fn initialize(
        contract: &AMM,
        exchange_bytecode_root: ContractId,
    ) -> CallResponse<()> {
        contract
            .methods()
            .initialize(exchange_bytecode_root)
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

pub mod test_helpers {
    use super::*;
    use amm_abi_calls::initialize;
    use exchange_abi_calls::constructor;
    use paths::{
        AMM_CONTRACT_BINARY_PATH, AMM_CONTRACT_STORAGE_PATH, EXCHANGE_CONTRACT_BINARY_PATH,
        MALICIOUS_EXCHANGE_CONTRACT_BINARY_PATH,
    };

    pub async fn bytecode_root() -> ContractId {
        // calculate the bytecode root of the exchange contract
        let exchange_raw_code = Contract::load_contract(
            EXCHANGE_CONTRACT_BINARY_PATH,
            &StorageConfiguration::default().storage_path,
        )
        .unwrap()
        .raw;
        (*TxContract::root_from_code(exchange_raw_code)).into()
    }

    pub async fn initialize_amm_contract(wallet: &WalletUnlocked, amm_instance: &AMM) {
        Contract::deploy(
            EXCHANGE_CONTRACT_BINARY_PATH,
            &wallet,
            TxParameters::default(),
            StorageConfiguration::default(),
        )
        .await
        .unwrap();

        initialize(amm_instance, bytecode_root().await).await;
    }

    pub async fn deploy_and_construct_exchange_contract(
        wallet: &WalletUnlocked,
        asset_pair: (ContractId, ContractId),
        malicious: Option<bool>,
        index_for_salt: Option<u8>,
    ) -> ExchangeContract {
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

        let exchange_instance = Exchange::new(exchange_contract_id.clone(), wallet.clone());
        constructor(&exchange_instance, asset_pair).await;
        ExchangeContract {
            contract_id: ContractId::new(*exchange_contract_id.hash()),
            bytecode_root: bytecode_root().await,
        }
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
        let (provider, _socket_addr) = setup_test_provider(coins.clone(), vec![], None, None).await;
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
        let amm_instance = AMM::new(amm_contract_id.clone(), wallet.clone());

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
