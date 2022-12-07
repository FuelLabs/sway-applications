use super::{
    abi::{
        amm::{add_pool, initialize},
        exchange::{add_liquidity, constructor, deposit},
        Exchange, AMM,
    },
    data_structures::{
        AMMContract, ExchangeContract, ExchangeContractConfiguration, LiquidityParameters,
        WalletAssetParameters,
    },
    paths::{
        AMM_CONTRACT_BINARY_PATH, AMM_CONTRACT_STORAGE_PATH, EXCHANGE_CONTRACT_BINARY_PATH,
        EXCHANGE_CONTRACT_STORAGE_PATH, MALICIOUS_EXCHANGE_CONTRACT_BINARY_PATH,
        MALICIOUS_EXCHANGE_CONTRACT_STORAGE_PATH,
    },
};
use fuels::{prelude::*, tx::Contract as TxContract};
use std::collections::HashMap;

pub mod common {
    use super::*;

    pub async fn exchange_bytecode_root() -> ContractId {
        let exchange_raw_code = Contract::load_contract(
            EXCHANGE_CONTRACT_BINARY_PATH,
            &StorageConfiguration::default().storage_path,
        )
        .unwrap()
        .raw;
        (*TxContract::root_from_code(exchange_raw_code)).into()
    }

    pub async fn setup_wallet_and_provider(
        asset_parameters: WalletAssetParameters,
    ) -> (WalletUnlocked, Vec<AssetId>, Provider) {
        let mut wallet = WalletUnlocked::new_random(None);

        let (coins, asset_ids) = setup_multiple_assets_coins(
            wallet.address(),
            asset_parameters.num_assets,
            asset_parameters.coins_per_asset,
            asset_parameters.amount_per_coin,
        );

        let (provider, _socket_addr) = setup_test_provider(coins.clone(), vec![], None, None).await;

        wallet.set_provider(provider.clone());

        (wallet, asset_ids, provider)
    }

    pub async fn deploy_amm(wallet: &WalletUnlocked) -> AMMContract {
        let contract_id = Contract::deploy(
            AMM_CONTRACT_BINARY_PATH,
            &wallet,
            TxParameters::default(),
            StorageConfiguration {
                storage_path: Option::Some(AMM_CONTRACT_STORAGE_PATH.to_string()),
                manual_storage_vec: Option::None,
            },
        )
        .await
        .unwrap();

        let instance = AMM::new(contract_id.clone(), wallet.clone());

        AMMContract {
            instance,
            id: contract_id.into(),
            pools: HashMap::new(),
        }
    }

    pub async fn deploy_and_initialize_amm(wallet: &WalletUnlocked) -> AMMContract {
        let amm = deploy_amm(&wallet).await;
        initialize(&amm.instance, exchange_bytecode_root().await).await;
        amm
    }

    pub async fn deploy_exchange(
        wallet: &WalletUnlocked,
        config: &ExchangeContractConfiguration,
    ) -> (ContractId, Exchange) {
        let binary_path = if config.malicious {
            MALICIOUS_EXCHANGE_CONTRACT_BINARY_PATH
        } else {
            EXCHANGE_CONTRACT_BINARY_PATH
        };
        let storage_path = if config.malicious {
            MALICIOUS_EXCHANGE_CONTRACT_STORAGE_PATH
        } else {
            EXCHANGE_CONTRACT_STORAGE_PATH
        }
        .to_string();

        let contract_id = Contract::deploy_with_parameters(
            binary_path,
            &wallet,
            TxParameters::default(),
            StorageConfiguration {
                storage_path: Option::Some(storage_path),
                manual_storage_vec: Option::None,
            },
            Salt::from(config.salt),
        )
        .await
        .unwrap();

        let id = ContractId::from(contract_id.clone());
        let instance = Exchange::new(contract_id, wallet.clone());

        (id, instance)
    }

    pub async fn deploy_and_construct_exchange(
        wallet: &WalletUnlocked,
        pair: (AssetId, AssetId),
        config: &ExchangeContractConfiguration,
    ) -> ExchangeContract {
        let (id, instance) = deploy_exchange(&wallet, &config).await;
        constructor(&instance, pair).await;
        ExchangeContract {
            bytecode_root: if config.compute_bytecode_root {
                Some(exchange_bytecode_root().await)
            } else {
                None
            },
            id,
            instance,
            pair,
        }
    }

    pub async fn deposit_both_assets(
        parameters: &LiquidityParameters,
        exchange: &ExchangeContract,
    ) {
        deposit(
            &exchange.instance,
            CallParameters::new(Some(parameters.amounts.0), Some(exchange.pair.0), None),
        )
        .await;

        deposit(
            &exchange.instance,
            CallParameters::new(Some(parameters.amounts.1), Some(exchange.pair.1), None),
        )
        .await;
    }

    pub async fn deposit_and_add_liquidity(
        liquidity_parameters: &LiquidityParameters,
        exchange: &ExchangeContract,
    ) -> u64 {
        deposit_both_assets(&liquidity_parameters, &exchange).await;

        add_liquidity(
            &exchange.instance,
            CallParameters::new(Some(0), None, None),
            TxParameters::new(None, Some(100_000_000), None),
            liquidity_parameters.liquidity,
            liquidity_parameters.deadline,
        )
        .await
        .value
    }
}

pub mod scripts {
    use super::*;
    use common::{
        deploy_and_construct_exchange, deploy_and_initialize_amm, deposit_and_add_liquidity,
        setup_wallet_and_provider,
    };

    const COINS_PER_ASSET: u64 = 100;
    const AMOUNT_PER_COIN: u64 = 1_000_000;
    pub const MAXIMUM_INPUT_AMOUNT: u64 = 1_000_000;

    pub async fn setup_exchange_contract(
        wallet: WalletUnlocked,
        pair: (AssetId, AssetId),
        ratio: u64,
        salt: [u8; 32],
    ) -> ExchangeContract {
        let exchange = deploy_and_construct_exchange(
            &wallet,
            pair,
            &ExchangeContractConfiguration::new(None, None, Some(salt)),
        )
        .await;

        deposit_and_add_liquidity(
            &LiquidityParameters::new(Some((100_000, 100_000 * ratio)), None, None),
            &exchange,
        )
        .await;

        exchange
    }

    pub async fn setup_exchange_contracts(
        wallet: WalletUnlocked,
        amm: &mut AMMContract,
        asset_ids: Vec<AssetId>,
    ) -> () {
        let mut i = 0;

        while i < asset_ids.len() - 1 {
            let asset_pair = (*asset_ids.get(i).unwrap(), *asset_ids.get(i + 1).unwrap());

            let exchange = setup_exchange_contract(
                wallet.clone(),
                asset_pair,
                (i as u64 + 1) * 3,
                [(i as u8 + 2); 32],
            )
            .await;

            add_pool(&amm.instance, asset_pair, exchange.id).await;

            amm.pools.insert(asset_pair, exchange);
            i += 1;
        }
    }

    pub async fn setup() -> (WalletUnlocked, Provider, AMMContract, Vec<AssetId>) {
        let asset_parameters = WalletAssetParameters {
            num_assets: 5,
            coins_per_asset: COINS_PER_ASSET,
            amount_per_coin: AMOUNT_PER_COIN,
        };
        let (wallet, asset_ids, provider) = setup_wallet_and_provider(asset_parameters).await;
        let mut amm = deploy_and_initialize_amm(&wallet).await;
        setup_exchange_contracts(wallet.clone(), &mut amm, asset_ids.clone()).await;
        (wallet, provider, amm, asset_ids)
    }
}
