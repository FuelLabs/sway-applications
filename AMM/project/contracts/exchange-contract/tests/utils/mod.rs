use fuels::prelude::*;
use test_utils::{
    abi::{exchange::balance, Exchange},
    data_structures::{
        ExchangeContract, ExchangeContractConfiguration, LiquidityParameters, WalletAssetParameters,
    },
    setup::common::{
        deploy_and_construct_exchange, deploy_exchange, deposit_and_add_liquidity,
        deposit_both_assets, setup_wallet_and_provider,
    },
};

pub struct ContractBalances {
    pub asset_a: u64,
    pub asset_b: u64,
}

pub struct WalletBalances {
    pub asset_a: u64,
    pub asset_b: u64,
    pub liquidity_pool_asset: u64,
}

const ASSET_PARAMETERS: WalletAssetParameters = WalletAssetParameters {
    num_assets: 3,
    coins_per_asset: 10,
    amount_per_coin: 1_000_000,
};

pub async fn contract_balances(exchange: &ExchangeContract) -> ContractBalances {
    let asset_a = balance(&exchange.instance, exchange.pair.0).await.value;
    let asset_b = balance(&exchange.instance, exchange.pair.1).await.value;
    ContractBalances { asset_a, asset_b }
}

pub async fn wallet_balances(
    exchange: &ExchangeContract,
    wallet: &WalletUnlocked,
) -> WalletBalances {
    let asset_a = wallet.get_asset_balance(&exchange.pair.0).await.unwrap();
    let asset_b = wallet.get_asset_balance(&exchange.pair.1).await.unwrap();
    let liquidity_pool_asset = wallet
        .get_asset_balance(&AssetId::new(*exchange.id))
        .await
        .unwrap();
    WalletBalances {
        asset_a,
        asset_b,
        liquidity_pool_asset,
    }
}

pub async fn setup() -> (Exchange, WalletUnlocked, AssetId, AssetId, AssetId, AssetId) {
    let (wallet, asset_ids, _provider) = setup_wallet_and_provider(ASSET_PARAMETERS).await;

    let (exchange_id, exchange_instance) =
        deploy_exchange(&wallet, &ExchangeContractConfiguration::default()).await;

    (
        exchange_instance,
        wallet,
        AssetId::from(*exchange_id),
        asset_ids[0],
        asset_ids[1],
        asset_ids[2],
    )
}

pub async fn setup_and_construct(
    deposit: bool,
    add_liquidity: bool,
) -> (
    ExchangeContract,
    WalletUnlocked,
    LiquidityParameters,
    AssetId,
) {
    let (wallet, asset_ids, _provider) = setup_wallet_and_provider(ASSET_PARAMETERS).await;

    let exchange = deploy_and_construct_exchange(
        &wallet,
        (asset_ids[0], asset_ids[1]),
        &ExchangeContractConfiguration::default(),
    )
    .await;

    let liquidity_parameters = LiquidityParameters::default();

    if deposit && add_liquidity {
        deposit_and_add_liquidity(&liquidity_parameters, &exchange).await;
    } else if deposit {
        deposit_both_assets(&liquidity_parameters, &exchange).await;
    }

    (exchange, wallet, liquidity_parameters, asset_ids[2])
}
