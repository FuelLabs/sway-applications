use fuels::{
    core::codec::EncoderConfig, prelude::{AssetId, WalletUnlocked}, types::Bits256
};
use test_utils::{
    data_structures::{
        AMMContract, SwapParameters, SwapResult, TransactionParameters, WalletAssetConfiguration,
    },
    interface::{
        exchange::preview_swap_exact_output, SwapExactOutputScript,
        SwapExactOutputScriptConfigurables,
    },
    paths::SWAP_EXACT_OUTPUT_SCRIPT_BINARY_PATH,
    setup::{
        common::{deploy_and_initialize_amm, setup_wallet_and_provider},
        scripts::{contract_instances, setup_exchange_contracts, transaction_inputs_outputs},
    },
};

pub async fn expected_swap_input(
    amm: &AMMContract,
    output_amount: u64,
    route: &Vec<AssetId>,
) -> u64 {
    assert!(route.len() >= 2);
    let (mut i, mut latest_input) = (route.len() - 1, output_amount);

    while i > 0 {
        let pair = (*route.get(i - 1).unwrap(), *route.get(i).unwrap());
        let exchange = &amm.pools.get(&pair).unwrap().instance;
        latest_input = preview_swap_exact_output(exchange, latest_input, pair.1, true)
            .await
            .other_asset
            .amount;
        i -= 1;
    }
    latest_input
}

pub async fn expected_and_actual_input(swap_parameters: SwapParameters) -> SwapResult {
    let (script_instance, amm, asset_ids, transaction_parameters, deadline) = setup().await;

    let mut route = Vec::with_capacity(swap_parameters.route_length as usize);
    let mut asset_index = 0;
    while asset_index < swap_parameters.route_length {
        route.push(*asset_ids.get(asset_index as usize).unwrap());
        asset_index += 1;
    }

    let expected = if swap_parameters.route_length >= 2 {
        Some(expected_swap_input(&amm, swap_parameters.amount, &route).await)
    } else {
        None
    };

    let actual = script_instance
        .main(
            route,
            swap_parameters.amount,
            expected.unwrap_or(0),
            deadline,
        )
        .with_contracts(&contract_instances(&amm))
        .with_inputs(transaction_parameters.inputs)
        .with_outputs(transaction_parameters.outputs)
        .call()
        .await
        .unwrap()
        .value;

    SwapResult { actual, expected }
}

pub async fn setup() -> (
    SwapExactOutputScript<WalletUnlocked>,
    AMMContract,
    Vec<AssetId>,
    TransactionParameters,
    u64,
) {
    let (wallet, asset_ids, provider) =
        setup_wallet_and_provider(&WalletAssetConfiguration::default()).await;

    let mut amm = deploy_and_initialize_amm(&wallet).await;

    setup_exchange_contracts(&wallet, &provider, &mut amm, &asset_ids).await;

    let mut contracts = vec![amm.id];
    contracts.extend(amm.pools.values().map(|exchange| exchange.id));

    let transaction_parameters =
        transaction_inputs_outputs(&wallet, &provider, &asset_ids, None).await;

    let deadline = provider.latest_block_height().await.unwrap() + 10;

    let script_configurables = SwapExactOutputScriptConfigurables::new(EncoderConfig::default())
        .with_AMM_ID(Bits256::from_hex_str(&amm.id.to_string()).unwrap()).unwrap();

    let script_instance = SwapExactOutputScript::new(wallet, SWAP_EXACT_OUTPUT_SCRIPT_BINARY_PATH)
        .with_configurables(script_configurables);

    (
        script_instance,
        amm,
        asset_ids,
        transaction_parameters,
        deadline.into(),
    )
}
