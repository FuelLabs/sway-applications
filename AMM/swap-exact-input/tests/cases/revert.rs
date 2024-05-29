use crate::utils::{expected_and_actual_output, expected_swap_output, setup};
use fuels::prelude::AssetId;
use test_utils::{data_structures::{SwapParameters, NUMBER_OF_ASSETS}, setup::scripts::contract_instances};

#[tokio::test]
#[should_panic(expected = "RouteTooShort")]
async fn when_route_length_is_zero() {
    expected_and_actual_output(SwapParameters {
        amount: 10,
        route_length: 0,
    })
    .await;
}

#[tokio::test]
#[should_panic(expected = "RouteTooShort")]
async fn when_route_length_is_one() {
    expected_and_actual_output(SwapParameters {
        amount: 10,
        route_length: 1,
    })
    .await;
}

#[tokio::test]
#[should_panic(expected = "PairExchangeNotRegistered")]
async fn when_pair_exchange_not_registered() {
    let (script_instance, amm, asset_ids, transaction_parameters, deadline) = setup().await;

    let mut route = asset_ids;
    let input_amount = 60;

    // make sure that the first asset in the route does not have a pool
    let not_registered_asset_id = AssetId::from([1u8; 32]);
    route.remove(0);
    route.insert(0, not_registered_asset_id);

    script_instance
        .main(route, input_amount, None, deadline)
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
    let input_amount = 60;

    let expected_result = expected_swap_output(&amm, input_amount, &route).await;

    script_instance
        .main(
            route,
            input_amount,
            Some(expected_result),
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
async fn when_minimum_output_not_satisfied() {
    let (script_instance, amm, asset_ids, transaction_parameters, deadline) = setup().await;

    let route = asset_ids;
    let input_amount = 60;

    let expected_result = expected_swap_output(&amm, input_amount, &route).await;

    script_instance
        .main(
            route,
            input_amount,
            Some(expected_result + 1), // setting the minimum to be higher than what it can be
            deadline,
        )
        .with_contracts(&contract_instances(&amm))
        .with_inputs(transaction_parameters.inputs)
        .with_outputs(transaction_parameters.outputs)
        .call()
        .await
        .unwrap();
}

#[tokio::test]
#[should_panic(expected = "ExpectedNonZeroAmount")]
async fn when_input_is_zero() {
    expected_and_actual_output(SwapParameters {
        amount: 0,
        route_length: 2,
    })
    .await;
}

#[tokio::test]
#[should_panic(expected = "ExpectedNonZeroAmount")]
// fails because starting with the second swap, the swap input is 0 which is not allowed
async fn when_input_is_one_and_route_has_more_than_two_assets() {
    expected_and_actual_output(SwapParameters {
        amount: 1,
        route_length: NUMBER_OF_ASSETS,
    })
    .await;
}
