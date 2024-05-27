use fuels::{
    core::codec::EncoderConfig, prelude::{AssetId, WalletUnlocked}, types::Bits256
};
use test_utils::{
    data_structures::{
        AMMContract, SwapParameters, SwapResult, TransactionParameters, WalletAssetConfiguration,
    },
    interface::{
        exchange::preview_swap_exact_input, SwapExactInputScript, SwapExactInputScriptConfigurables,
    },
    paths::SWAP_EXACT_INPUT_SCRIPT_BINARY_PATH,
    setup::{
        common::{deploy_and_initialize_amm, setup_wallet_and_provider},
        scripts::{contract_instances, setup_exchange_contracts, transaction_inputs_outputs},
    },
};

pub async fn expected_swap_output(
    amm: &AMMContract,
    input_amount: u64,
    route: &Vec<AssetId>,
) -> u64 {
    assert!(route.len() >= 2);
    let (mut i, mut latest_output) = (0, input_amount);

    while i < route.len() - 1 {
        let pair = (*route.get(i).unwrap(), *route.get(i + 1).unwrap());
        let exchange = &amm.pools.get(&pair).unwrap().instance;
        latest_output = preview_swap_exact_input(exchange, latest_output, pair.0, true)
            .await
            .other_asset
            .amount;
        i += 1;
    }
    latest_output
}

pub async fn expected_and_actual_output(swap_parameters: SwapParameters) -> SwapResult {
    let (script_instance, amm, asset_ids, transaction_parameters, deadline) = setup().await;

    let mut route = Vec::with_capacity(swap_parameters.route_length as usize);
    let mut asset_index = 0;
    while asset_index < swap_parameters.route_length {
        route.push(*asset_ids.get(asset_index as usize).unwrap());
        asset_index += 1;
    }

    let expected = if swap_parameters.route_length >= 2 {
        Some(expected_swap_output(&amm, swap_parameters.amount, &route).await)
    } else {
        None
    };

    let actual = script_instance
        .main(route, swap_parameters.amount, expected, deadline)
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
    SwapExactInputScript<WalletUnlocked>,
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

    let script_configurables = SwapExactInputScriptConfigurables::new(EncoderConfig::default())
        .with_AMM_ID(Bits256::from_hex_str(&amm.id.to_string()).unwrap()).unwrap();

    let script_instance = SwapExactInputScript::new(wallet, SWAP_EXACT_INPUT_SCRIPT_BINARY_PATH)
        .with_configurables(script_configurables);

    (
        script_instance,
        amm,
        asset_ids,
        transaction_parameters,
        deadline.into(),
    )
}
