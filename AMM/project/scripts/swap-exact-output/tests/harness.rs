use fuels::prelude::*;
use test_utils::{
    abi::{exchange::preview_swap_exact_output, SwapExactOutputScript},
    data_structures::AMMContract,
    paths::SWAP_EXACT_OUTPUT_SCRIPT_BINARY_PATH,
    setup::scripts::setup,
    transaction::transaction_inputs_outputs_for_scripts,
};

pub async fn expected_swap_amounts(
    amm: &AMMContract,
    output_amount: u64,
    route: &Vec<AssetId>,
) -> Vec<u64> {
    assert!(route.len() >= 2);
    let (mut i, mut amounts) = (route.len() - 1, vec![output_amount]);

    while i > 0 {
        let pair = (*route.get(i - 1).unwrap(), *route.get(i).unwrap());
        let exchange = &amm.pools.get(&pair).unwrap().instance;
        let amount = preview_swap_exact_output(&exchange, amounts[0], pair.1)
            .await
            .value
            .amount;
        amounts.insert(0, amount);
        i -= 1;
    }
    amounts
}

#[tokio::test]
async fn can_swap_exact_output_along_route() {
    let (wallet, provider, amm, asset_ids) = setup().await;

    let (inputs, outputs) =
        transaction_inputs_outputs_for_scripts(&wallet, &provider, &amm, &asset_ids).await;

    let route = asset_ids;
    let script_instance =
        SwapExactOutputScript::new(wallet.clone(), SWAP_EXACT_OUTPUT_SCRIPT_BINARY_PATH);
    let input_amount: u64 = 10_000;

    let amounts = expected_swap_amounts(&amm, input_amount, &route).await;
    let expected_result = amounts[0];

    let result = script_instance
        .main(
            amm.id,
            route
                .clone()
                .into_iter()
                .map(|asset_id| ContractId::new(*asset_id))
                .collect(),
            amounts,
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
