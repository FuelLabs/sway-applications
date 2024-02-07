use crate::utils::{expected_and_actual_input, expected_swap_input, setup};
use fuels::prelude::AssetId;
use test_utils::{
    data_structures::{SwapParameters, NUMBER_OF_ASSETS},
    setup::scripts::contract_instances,
};

#[tokio::test]
#[should_panic(expected = "RouteTooShort")]
async fn when_route_length_is_zero() {
    expected_and_actual_input(SwapParameters {
        amount: 0,
        route_length: 0,
    })
    .await;
}

#[tokio::test]
#[should_panic(expected = "RouteTooShort")]
async fn when_route_length_is_one() {
    expected_and_actual_input(SwapParameters {
        amount: 0,
        route_length: 1,
    })
    .await;
}

#[tokio::test]
#[should_panic(expected = "PairExchangeNotRegistered")]
async fn when_pair_exchange_not_registered() {
    let (script_instance, amm, asset_ids, transaction_parameters, deadline) = setup().await;

    let mut route = asset_ids;
    let output_amount = 10_000;
    let maximum_input_amount = 0;

    // make sure that the first asset in the route does not have a pool
    let not_registered_asset_id = AssetId::from([1u8; 32]);
    route.remove(route.len() - 1);
    route.push(not_registered_asset_id);

    script_instance
        .main(route, output_amount, maximum_input_amount, deadline)
        .with_contracts(&contract_instances(&amm))
        .with_inputs(transaction_parameters.inputs)
        .with_outputs(transaction_parameters.outputs)
        .call()
        .await
        .unwrap();
}

#[tokio::test]
#[should_panic(expected = "DeadlinePassed")]
async fn when_deadline_passed() {
    let (script_instance, amm, asset_ids, transaction_parameters, _deadline) = setup().await;

    let route = asset_ids;
    let output_amount = 10_000;
    let maximum_input_amount = expected_swap_input(&amm, output_amount, &route).await;

    script_instance
        .main(
            route,
            output_amount,
            maximum_input_amount,
            0, // deadline is 0
        )
        .with_contracts(&contract_instances(&amm))
        .with_inputs(transaction_parameters.inputs)
        .with_outputs(transaction_parameters.outputs)
        .call()
        .await
        .unwrap();
}

#[tokio::test]
#[should_panic(expected = "ExcessiveSlippage")]
async fn when_maximum_input_not_satisfied() {
    let (script_instance, amm, asset_ids, transaction_parameters, deadline) = setup().await;

    let route = asset_ids;
    let output_amount = 10_000;
    let maximum_input_amount = expected_swap_input(&amm, output_amount, &route).await;

    script_instance
        .main(
            route,
            output_amount,
            maximum_input_amount - 1, // setting the maximum to be lower than what it can be
            deadline,
        )
        .with_contracts(&contract_instances(&amm))
        .with_inputs(transaction_parameters.inputs)
        .with_outputs(transaction_parameters.outputs)
        .call()
        .await
        .unwrap();
}

#[should_panic(expected = "DesiredAmountTooLow")]
#[tokio::test]
async fn when_requested_swap_output_is_zero() {
    expected_and_actual_input(SwapParameters {
        amount: 0,
        route_length: 2,
    })
    .await;
}

#[should_panic(expected = "DesiredAmountTooLow")]
#[tokio::test]
async fn when_requested_swap_output_is_too_low() {
    expected_and_actual_input(SwapParameters {
        amount: 1,
        route_length: NUMBER_OF_ASSETS,
    })
    .await;
}
