use fuels::prelude::WalletUnlocked;
use test_utils::{
    data_structures::{
        ExchangeContract, ExchangeContractConfiguration, LiquidityParameters,
        TransactionParameters, WalletAssetConfiguration,
    },
    interface::{
        exchange::{deposit, preview_add_liquidity, withdraw},
        AtomicAddLiquidityScript,
    },
    paths::ATOMIC_ADD_LIQUIDITY_SCRIPT_BINARY_PATH,
    setup::{
        common::{deploy_and_construct_exchange, setup_wallet_and_provider},
        scripts::transaction_inputs_outputs,
    },
};

pub async fn expected_liquidity(
    exchange: &ExchangeContract,
    liquidity_parameters: &LiquidityParameters,
    override_asset: bool,
) -> u64 {
    deposit(
        &exchange.instance,
        if override_asset {
            liquidity_parameters.amounts.1
        } else {
            liquidity_parameters.amounts.0
        },
        if override_asset {
            exchange.pair.1
        } else {
            exchange.pair.0
        },
    )
    .await;

    let preview_add_liquidity_info = preview_add_liquidity(
        &exchange.instance,
        if override_asset {
            liquidity_parameters.amounts.0
        } else {
            liquidity_parameters.amounts.1
        },
        if override_asset {
            exchange.pair.0
        } else {
            exchange.pair.1
        },
        true,
    )
    .await;

    withdraw(
        &exchange.instance,
        if override_asset {
            liquidity_parameters.amounts.1
        } else {
            liquidity_parameters.amounts.0
        },
        if override_asset {
            exchange.pair.1
        } else {
            exchange.pair.0
        },
    )
    .await;

    preview_add_liquidity_info.liquidity_asset_to_receive.amount
}

pub async fn setup(
    deposit_amounts: (u64, u64),
    liquidity: u64,
) -> (
    AtomicAddLiquidityScript<WalletUnlocked>,
    ExchangeContract,
    LiquidityParameters,
    TransactionParameters,
) {
    let (wallet, asset_ids, provider) =
        setup_wallet_and_provider(&WalletAssetConfiguration::default()).await;

    let exchange = deploy_and_construct_exchange(
        &wallet,
        &ExchangeContractConfiguration::new(Some((asset_ids[0], asset_ids[1])), None, None, None),
    )
    .await;

    let liquidity_parameters = LiquidityParameters::new(
        Some(deposit_amounts),
        Some((provider.latest_block_height().await.unwrap() + 10).into()),
        Some(liquidity),
    );

    let transaction_parameters = transaction_inputs_outputs(
        &wallet,
        &provider,
        &[*asset_ids.first().unwrap(), *asset_ids.get(1).unwrap()],
        Some(&vec![
            liquidity_parameters.amounts.0,
            liquidity_parameters.amounts.1,
        ]),
    )
    .await;

    let script_instance =
        AtomicAddLiquidityScript::new(wallet, ATOMIC_ADD_LIQUIDITY_SCRIPT_BINARY_PATH);

    (
        script_instance,
        exchange,
        liquidity_parameters,
        transaction_parameters,
    )
}
