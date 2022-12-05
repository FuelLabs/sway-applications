use super::{
    abi::{
        amm::{add_pool, initialize},
        exchange::{add_liquidity, constructor, deposit},
    },
    amounts::{AMOUNT_PER_COIN, COINS_PER_ASSET},
    paths::{AMM_CONTRACT_STORAGE_PATH, EXCHANGE_CONTRACT_STORAGE_PATH},
};
use crate::utils::{
    paths::{AMM_CONTRACT_BINARY_PATH, EXCHANGE_CONTRACT_BINARY_PATH},
    Exchange, AMM,
};
use fuels::{prelude::*, tx::Contract as TxContract};
use std::collections::HashMap;

pub struct AMMContract {
    pub instance: AMM,
    pub id: ContractId,
    pub pools: HashMap<(AssetId, AssetId), ExchangeContract>,
}

pub struct ExchangeContract {
    pub instance: Exchange,
    pub id: ContractId,
}

pub struct LiquidityParameters {
    pub asset_a: AssetId,
    pub asset_b: AssetId,
    pub amount_a: u64,
    pub amount_b: u64,
    pub liquidity: u64,
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

// TODO (@supiket): call the atomic add liquidity script for test setup
pub async fn deposit_and_add_liquidity(exchange_instance: &Exchange, amounts: &LiquidityParameters) -> u64 {
    let call_params =
        CallParameters::new(Some(amounts.amount_a), Some(amounts.asset_a.clone()), None);
    deposit(&exchange_instance, call_params).await;

    let call_params =
        CallParameters::new(Some(amounts.amount_b), Some(amounts.asset_b.clone()), None);
    deposit(&exchange_instance, call_params).await;

    let result = add_liquidity(&exchange_instance, amounts.liquidity, 1000).await;

    result.value
}

pub async fn setup_wallet_and_provider() -> (WalletUnlocked, Vec<AssetId>, Provider) {
    let mut wallet = WalletUnlocked::new_random(None);
    let num_assets = 5;
    let coins_per_asset = COINS_PER_ASSET;
    let amount_per_coin = AMOUNT_PER_COIN;

    let (coins, asset_ids) = setup_multiple_assets_coins(
        wallet.address(),
        num_assets,
        coins_per_asset,
        amount_per_coin,
    );

    let (provider, _socket_addr) = setup_test_provider(coins.clone(), vec![], None, None).await;

    wallet.set_provider(provider.clone());

    (wallet, asset_ids, provider)
}

pub async fn setup_amm_contract(wallet: WalletUnlocked) -> AMMContract {
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

    initialize(&instance, exchange_bytecode_root().await).await;

    AMMContract {
        instance,
        id: contract_id.into(),
        pools: HashMap::new(),
    }
}

pub async fn setup_exchange_contract(
    wallet: WalletUnlocked,
    pair: (AssetId, AssetId),
    ratio: u64,
    salt: [u8; 32],
) -> ExchangeContract {
    let contract_id = Contract::deploy_with_parameters(
        EXCHANGE_CONTRACT_BINARY_PATH,
        &wallet,
        TxParameters::default(),
        StorageConfiguration {
            storage_path: Option::Some(EXCHANGE_CONTRACT_STORAGE_PATH.to_string()),
            manual_storage_vec: Option::None,
        },
        Salt::from(salt),
    )
    .await
    .unwrap();

    let id = ContractId::from(contract_id.clone());
    let instance = Exchange::new(contract_id, wallet.clone());

    constructor(&instance, pair).await;

    let amounts = LiquidityParameters {
        asset_a: pair.0,
        amount_a: 100,
        asset_b: pair.1,
        amount_b: 100 * ratio,
        liquidity: 100,
    };

    deposit_and_add_liquidity(&instance, &amounts).await;

    ExchangeContract { instance, id }
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
            (i as u64 + 1) * 6,
            [(i as u8 + 2); 32],
        )
        .await;

        add_pool(&amm.instance, asset_pair, exchange.id).await;

        amm.pools.insert(asset_pair, exchange);
        i += 1;
    }
}

pub async fn setup() -> (WalletUnlocked, Provider, AMMContract, Vec<AssetId>) {
    let (wallet, asset_ids, provider) = setup_wallet_and_provider().await;
    let mut amm = setup_amm_contract(wallet.clone()).await;
    setup_exchange_contracts(wallet.clone(), &mut amm, asset_ids.clone()).await;
    (wallet, provider, amm, asset_ids)
}
