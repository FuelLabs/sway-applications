use fuel_gql_client::prelude::*;
use fuels::prelude::*;
use test_utils::{
    abi::{exchange::preview_swap_exact_input, SwapScript},
    data_structures::AMMContract,
    paths::SWAP_SCRIPT_BINARY_PATH,
    setup::scripts::setup,
    transaction::transaction_inputs_outputs_for_scripts,
};

pub async fn expected_swap_output(
    amm: &AMMContract,
    input_amount: u64,
    route: &Vec<AssetId>,
) -> u64 {
    let mut expected_output = input_amount;
    let mut i = 0;
    while i < route.len() - 1 {
        let pair = (*route.get(i).unwrap(), *route.get(i + 1).unwrap());
        let exchange = &amm.pools.get(&pair).unwrap().instance;
        expected_output = preview_swap_exact_input(&exchange, expected_output, pair.0)
            .await
            .value
            .amount;

        i += 1;
    }
    expected_output
}

#[tokio::test]
async fn can_swap_exact_input_along_route() {
    let (wallet, provider, amm, asset_ids) = setup().await;

    let (inputs, outputs) =
        transaction_inputs_outputs_for_scripts(&wallet, &provider, &amm, &asset_ids).await;

    let route = asset_ids;
    let script_instance = SwapScript::new(wallet.clone(), SWAP_SCRIPT_BINARY_PATH);
    let input_amount: u64 = 60;

    let expected_result = expected_swap_output(&amm, input_amount, &route).await;

    let result = script_instance
        .main(
            amm.id,
            route
                .clone()
                .into_iter()
                .map(|asset_id| ContractId::new(*asset_id))
                .collect(),
            input_amount,
        )
        .with_inputs(inputs)
        .with_outputs(outputs)
        .tx_params(TxParameters::new(None, Some(100_000_000), None))
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(expected_result, result);
}
