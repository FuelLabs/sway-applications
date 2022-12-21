use fuels::prelude::*;
use test_utils::{
    abi::exchange::{deposit, preview_add_liquidity, withdraw},
    data_structures::{
        ExchangeContract, ExchangeContractConfiguration, LiquidityParameters,
        TransactionParameters, WalletAssetConfiguration,
    },
    setup::{
        common::{deploy_and_construct_exchange, setup_wallet_and_provider},
        scripts::transaction_inputs_outputs,
    },
};

pub async fn expected_liquidity(
    exchange: &ExchangeContract,
    liquidity_parameters: &LiquidityParameters,
) -> u64 {
    deposit(
        &exchange.instance,
        liquidity_parameters.amounts.0,
        exchange.pair.0,
    )
    .await;

    let preview_add_liquidity_info = preview_add_liquidity(
        &exchange.instance,
        liquidity_parameters.amounts.1,
        exchange.pair.1,
        true,
    )
    .await;

    withdraw(
        &exchange.instance,
        liquidity_parameters.amounts.0,
        exchange.pair.0,
    )
    .await;

    preview_add_liquidity_info.liquidity_asset_amount_to_receive
}

pub async fn setup(
    deposit_amounts: (u64, u64),
    liquidity: u64,
) -> (
    WalletUnlocked,
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
        Some(provider.latest_block_height().await.unwrap() + 10),
        Some(liquidity),
    );

    let transaction_parameters = transaction_inputs_outputs(
        &wallet,
        &provider,
        &vec![exchange.id],
        &vec![*asset_ids.get(0).unwrap(), *asset_ids.get(1).unwrap()],
        Some(&vec![
            liquidity_parameters.amounts.0,
            liquidity_parameters.amounts.1,
        ]),
    )
    .await;

    (
        wallet,
        exchange,
        liquidity_parameters,
        transaction_parameters,
    )
}
