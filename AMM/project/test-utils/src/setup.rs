use super::{
    data_structures::{
        AMMContract, ExchangeContract, ExchangeContractConfiguration, LiquidityParameters,
        TransactionParameters, WalletAssetConfiguration,
    },
    interface::{
        amm::{add_pool, initialize},
        exchange::{add_liquidity, constructor, deposit},
        Exchange, AMM,
    },
    paths::{
        AMM_CONTRACT_BINARY_PATH, AMM_CONTRACT_STORAGE_PATH, EXCHANGE_CONTRACT_BINARY_PATH,
        EXCHANGE_CONTRACT_STORAGE_PATH, MALICIOUS_EXCHANGE_CONTRACT_BINARY_PATH,
        MALICIOUS_EXCHANGE_CONTRACT_STORAGE_PATH,
    },
    transaction::{
        transaction_input_coin, transaction_input_contract, transaction_output_contract,
        transaction_output_variable,
    },
};
use fuels::{
    prelude::*,
    tx::{Contract as TxContract, Input, Output},
};
use std::collections::HashMap;

pub mod common {
    use super::*;

    pub async fn deploy_amm(wallet: &WalletUnlocked) -> AMMContract {
        let contract_id = Contract::deploy(
            AMM_CONTRACT_BINARY_PATH,
            &wallet,
            TxParameters::default(),
            StorageConfiguration {
                storage_path: Some(AMM_CONTRACT_STORAGE_PATH.to_string()),
                manual_storage_vec: None,
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

    pub async fn deploy_and_construct_exchange(
        wallet: &WalletUnlocked,
        config: &ExchangeContractConfiguration,
    ) -> ExchangeContract {
        let (id, instance) = deploy_exchange(&wallet, &config).await;

        constructor(&instance, config.pair).await;

        ExchangeContract {
            bytecode_root: if config.compute_bytecode_root {
                Some(exchange_bytecode_root().await)
            } else {
                None
            },
            id,
            instance,
            pair: config.pair,
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
                storage_path: Some(storage_path),
                manual_storage_vec: None,
            },
            Salt::from(config.salt),
        )
        .await
        .unwrap();

        let id = ContractId::from(contract_id.clone());
        let instance = Exchange::new(contract_id, wallet.clone());

        (id, instance)
    }

    // TODO: once the script is reliable enough, use it for this functionality
    pub async fn deposit_and_add_liquidity(
        liquidity_parameters: &LiquidityParameters,
        exchange: &ExchangeContract,
        override_gas_limit: bool,
    ) -> u64 {
        deposit(
            &exchange.instance,
            liquidity_parameters.amounts.0,
            exchange.pair.0,
        )
        .await;

        deposit(
            &exchange.instance,
            liquidity_parameters.amounts.1,
            exchange.pair.1,
        )
        .await;

        add_liquidity(
            &exchange.instance,
            liquidity_parameters.liquidity,
            liquidity_parameters.deadline,
            override_gas_limit,
        )
        .await
    }

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
        asset_parameters: &WalletAssetConfiguration,
    ) -> (WalletUnlocked, Vec<AssetId>, Provider) {
        let mut wallet = WalletUnlocked::new_random(None);

        let (coins, asset_ids) = setup_multiple_assets_coins(
            wallet.address(),
            asset_parameters.number_of_assets,
            asset_parameters.coins_per_asset,
            asset_parameters.amount_per_coin,
        );

        let (provider, _socket_addr) = setup_test_provider(coins.clone(), vec![], None, None).await;

        wallet.set_provider(provider.clone());

        (wallet, asset_ids, provider)
    }
}

pub mod scripts {
    use super::*;
    use common::{deploy_and_construct_exchange, deposit_and_add_liquidity};

    pub const MAXIMUM_INPUT_AMOUNT: u64 = 1_000_000;

    pub async fn setup_exchange_contract(
        wallet: &WalletUnlocked,
        exchange_config: &ExchangeContractConfiguration,
        liquidity_parameters: &LiquidityParameters,
    ) -> ExchangeContract {
        let exchange = deploy_and_construct_exchange(&wallet, &exchange_config).await;

        deposit_and_add_liquidity(&liquidity_parameters, &exchange, false).await;

        exchange
    }

    pub async fn setup_exchange_contracts(
        wallet: &WalletUnlocked,
        provider: &Provider,
        amm: &mut AMMContract,
        asset_ids: &Vec<AssetId>,
    ) -> () {
        let mut exchange_index = 0;

        while exchange_index < asset_ids.len() - 1 {
            let asset_pair = (
                *asset_ids.get(exchange_index).unwrap(),
                *asset_ids.get(exchange_index + 1).unwrap(),
            );

            let exchange = setup_exchange_contract(
                wallet,
                &ExchangeContractConfiguration::new(
                    Some(asset_pair),
                    None,
                    None,
                    Some([(exchange_index as u8 + 2); 32]),
                ),
                &LiquidityParameters::new(
                    Some((100_000, 100_000 * (exchange_index as u64 + 1) * 3)),
                    Some(provider.latest_block_height().await.unwrap() + 10),
                    Some(100_000),
                ),
            )
            .await;

            add_pool(&amm.instance, asset_pair, exchange.id).await;

            amm.pools.insert(asset_pair, exchange);
            exchange_index += 1;
        }
    }

    pub async fn transaction_inputs_outputs(
        wallet: &WalletUnlocked,
        provider: &Provider,
        contracts: &Vec<ContractId>,
        assets: &Vec<AssetId>,
        amounts: Option<&Vec<u64>>,
    ) -> TransactionParameters {
        let mut input_contracts: Vec<Input> = vec![];
        let mut output_contracts: Vec<Output> = vec![];

        contracts
            .into_iter()
            .enumerate()
            .for_each(|(index, contract_id)| {
                input_contracts.push(transaction_input_contract(*contract_id));
                output_contracts.push(transaction_output_contract(index as u8));
            });

        let mut input_coins: Vec<Input> = vec![];
        let mut output_variables: Vec<Output> = vec![];

        let mut asset_index = 0;
        while asset_index < assets.len() {
            input_coins.extend(
                transaction_input_coin(
                    &provider,
                    wallet.address(),
                    *assets.get(asset_index).unwrap(),
                    if amounts.is_some() {
                        *amounts.unwrap().get(asset_index).unwrap()
                    } else {
                        MAXIMUM_INPUT_AMOUNT
                    },
                )
                .await,
            );
            output_variables.push(transaction_output_variable());
            asset_index += 1;
        }

        TransactionParameters {
            inputs: [input_contracts, input_coins].concat(),
            outputs: [output_contracts, output_variables].concat(),
        }
    }
}
