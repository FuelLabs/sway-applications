use crate::utils::expected_and_actual_output;
use test_utils::data_structures::{SwapParameters, NUMBER_OF_ASSETS};

#[tokio::test]
async fn can_swap_exact_input_along_route_small_input() {
    let swap_result = expected_and_actual_output(SwapParameters {
        amount: 2,
        route_length: NUMBER_OF_ASSETS,
    })
    .await;

    assert_eq!(swap_result.expected.unwrap(), swap_result.actual);
}

#[tokio::test]
async fn can_swap_exact_input_along_route_middle_input() {
    let swap_result = expected_and_actual_output(SwapParameters {
        amount: 60,
        route_length: NUMBER_OF_ASSETS,
    })
    .await;

    assert_eq!(swap_result.expected.unwrap(), swap_result.actual);
}

#[tokio::test]
async fn can_swap_exact_input_along_route_large_input() {
    let swap_result = expected_and_actual_output(SwapParameters {
        amount: 10_000,
        route_length: NUMBER_OF_ASSETS,
    })
    .await;

    assert_eq!(swap_result.expected.unwrap(), swap_result.actual);
}

#[tokio::test]
async fn can_swap_exact_input_two_assets_small_input() {
    let swap_result = expected_and_actual_output(SwapParameters {
        amount: 1,
        route_length: 2,
    })
    .await;

    assert_eq!(swap_result.expected.unwrap(), swap_result.actual);
}

#[tokio::test]
async fn can_swap_exact_input_two_assets_middle_input() {
    let swap_result = expected_and_actual_output(SwapParameters {
        amount: 60,
        route_length: 2,
    })
    .await;

    assert_eq!(swap_result.expected.unwrap(), swap_result.actual);
}

#[tokio::test]
async fn can_swap_exact_input_two_assets_large_input() {
    let swap_result = expected_and_actual_output(SwapParameters {
        amount: 10_000,
        route_length: 2,
    })
    .await;

    assert_eq!(swap_result.expected.unwrap(), swap_result.actual);
}
