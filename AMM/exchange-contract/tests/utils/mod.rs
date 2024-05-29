use fuels::{
    accounts::ViewOnlyAccount,
    prelude::{AssetId, WalletUnlocked},
    tx::ContractIdExt,
    types::Bytes32,
};
use test_utils::{
    data_structures::{
        ExchangeContract, ExchangeContractConfiguration, LiquidityParameters,
        WalletAssetConfiguration,
    },
    interface::{
        exchange::{balance, deposit},
        Exchange,
    },
    setup::common::{
        deploy_and_construct_exchange, deploy_exchange, deposit_and_add_liquidity,
        setup_wallet_and_provider,
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

pub struct Assets {
    pub asset_1: AssetId,
    pub asset_2: AssetId,
    pub asset_3: AssetId,
    pub liquidity_pool_asset: AssetId,
}

pub fn maximum_input_for_exact_output(
    output_amount: u64,
    input_reserve: u64,
    output_reserve: u64,
    liquidity_miner_fee: u64,
) -> u64 {
    let numerator = input_reserve * output_amount;
    let denominator = (output_reserve - output_amount) * (1 - (1 / liquidity_miner_fee));
    (numerator / denominator) + 1
}

pub fn minimum_output_given_exact_input(
    input_amount: u64,
    input_reserve: u64,
    output_reserve: u64,
    liquidity_miner_fee: u64,
) -> u64 {
    let input_amount_with_fee = input_amount * (1 - (1 / liquidity_miner_fee));
    let numerator = input_amount_with_fee * output_reserve;
    let denominator = input_reserve + input_amount_with_fee;
    numerator / denominator
}

pub async fn contract_balances(exchange: &ExchangeContract) -> ContractBalances {
    let asset_a = balance(&exchange.instance, exchange.pair.0).await;
    let asset_b = balance(&exchange.instance, exchange.pair.1).await;
    ContractBalances { asset_a, asset_b }
}

pub async fn wallet_balances(
    exchange: &ExchangeContract,
    wallet: &WalletUnlocked,
) -> WalletBalances {
    let asset_a = wallet.get_asset_balance(&exchange.pair.0).await.unwrap();
    let asset_b = wallet.get_asset_balance(&exchange.pair.1).await.unwrap();
    let liquidity_pool_asset = wallet
        .get_asset_balance(&exchange.id.asset_id(&Bytes32::zeroed()))
        .await
        .unwrap();
    WalletBalances {
        asset_a,
        asset_b,
        liquidity_pool_asset,
    }
}

pub async fn setup() -> (Exchange<WalletUnlocked>, WalletUnlocked, Assets, u64) {
    let (wallet, asset_ids, provider) =
        setup_wallet_and_provider(&WalletAssetConfiguration::default()).await;

    let (exchange_id, exchange_instance) = deploy_exchange(
        &wallet,
        &ExchangeContractConfiguration::new(None, None, None, None),
    )
    .await;

    let assets = Assets {
        asset_1: asset_ids[0],
        asset_2: asset_ids[1],
        asset_3: asset_ids[2],
        liquidity_pool_asset: AssetId::from(*exchange_id),
    };

    let deadline = provider.latest_block_height().await.unwrap() + 5;

    (exchange_instance, wallet, assets, deadline.into())
}

pub async fn setup_and_construct(
    deposit_both: bool,
    add_liquidity: bool,
) -> (
    ExchangeContract,
    WalletUnlocked,
    LiquidityParameters,
    AssetId,
) {
    let (wallet, asset_ids, provider) =
        setup_wallet_and_provider(&WalletAssetConfiguration::default()).await;

    let exchange = deploy_and_construct_exchange(
        &wallet,
        &ExchangeContractConfiguration::new(Some((asset_ids[0], asset_ids[1])), None, None, None),
    )
    .await;

    let liquidity_parameters = LiquidityParameters::new(
        Some((10000, 40000)),
        Some((provider.latest_block_height().await.unwrap() + 20).into()),
        Some(20000),
    );

    if deposit_both && add_liquidity {
        deposit_and_add_liquidity(&liquidity_parameters, &exchange, false).await;
    } else if deposit_both {
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
    }

    (exchange, wallet, liquidity_parameters, asset_ids[2])
}
