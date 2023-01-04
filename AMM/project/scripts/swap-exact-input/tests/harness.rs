use fuels::prelude::*;
use test_utils::{
    data_structures::{
        AMMContract, SwapParameters, SwapResult, TransactionParameters, WalletAssetConfiguration,
        NUMBER_OF_ASSETS,
    },
    interface::{exchange::preview_swap_exact_input, SwapExactInputScript, SCRIPT_GAS_LIMIT},
    paths::SWAP_EXACT_INPUT_SCRIPT_BINARY_PATH,
    setup::{
        common::{deploy_and_initialize_amm, setup_wallet_and_provider},
        scripts::{setup_exchange_contracts, transaction_inputs_outputs},
    },
};

async fn expected_swap_output(amm: &AMMContract, input_amount: u64, route: &Vec<AssetId>) -> u64 {
    assert!(route.len() >= 2);
    let (mut i, mut latest_output) = (0, input_amount);

    while i < route.len() - 1 {
        let pair = (*route.get(i).unwrap(), *route.get(i + 1).unwrap());
        let exchange = &amm.pools.get(&pair).unwrap().instance;
        latest_output = preview_swap_exact_input(&exchange, latest_output, pair.0, true)
            .await
            .other_asset
            .amount;
        i += 1;
    }
    latest_output
}

async fn expected_and_actual_output(swap_parameters: SwapParameters) -> SwapResult {
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
        .main(
            route
                .into_iter()
                .map(|asset_id| ContractId::new(*asset_id))
                .collect(),
            swap_parameters.amount,
            expected,
            deadline,
        )
        .with_inputs(transaction_parameters.inputs)
        .with_outputs(transaction_parameters.outputs)
        .tx_params(TxParameters::new(None, Some(SCRIPT_GAS_LIMIT), None))
        .call()
        .await
        .unwrap()
        .value;

    SwapResult { actual, expected }
}

async fn setup() -> (
    SwapExactInputScript,
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
    contracts.extend(amm.pools.values().into_iter().map(|exchange| exchange.id));

    let transaction_parameters =
        transaction_inputs_outputs(&wallet, &provider, &contracts, &asset_ids, None).await;

    let deadline = provider.latest_block_height().await.unwrap() + 10;

    let script_instance = SwapExactInputScript::new(wallet, SWAP_EXACT_INPUT_SCRIPT_BINARY_PATH);

    (
        script_instance,
        amm,
        asset_ids,
        transaction_parameters,
        deadline,
    )
}

mod success {
    use super::*;

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
}

mod revert {
    use super::*;

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
        let (script_instance, _amm, asset_ids, transaction_parameters, deadline) = setup().await;

        let mut route = asset_ids;
        let input_amount = 60;

        // make sure that the first asset in the route does not have a pool
        let not_registered_asset_id = AssetId::from([1u8; 32]);
        route.remove(0);
        route.insert(0, not_registered_asset_id);

        script_instance
            .main(
                route
                    .into_iter()
                    .map(|asset_id| ContractId::new(*asset_id))
                    .collect(),
                input_amount,
                None,
                deadline,
            )
            .with_inputs(transaction_parameters.inputs)
            .with_outputs(transaction_parameters.outputs)
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(18446744073709486080)")]
    // the contract call in the script fails with "DeadlinePassed" but that message is not propagated
    async fn when_deadline_passed() {
        let (script_instance, amm, asset_ids, transaction_parameters, _deadline) = setup().await;

        let route = asset_ids;
        let input_amount = 60;

        let expected_result = expected_swap_output(&amm, input_amount, &route).await;

        script_instance
            .main(
                route
                    .into_iter()
                    .map(|asset_id| ContractId::new(*asset_id))
                    .collect(),
                input_amount,
                Some(expected_result),
                0, // deadline is 0
            )
            .with_inputs(transaction_parameters.inputs)
            .with_outputs(transaction_parameters.outputs)
            .tx_params(TxParameters::new(None, Some(SCRIPT_GAS_LIMIT), None))
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
                route
                    .into_iter()
                    .map(|asset_id| ContractId::new(*asset_id))
                    .collect(),
                input_amount,
                Some(expected_result + 1), // setting the minimum to be higher than what it can be
                deadline,
            )
            .with_inputs(transaction_parameters.inputs)
            .with_outputs(transaction_parameters.outputs)
            .tx_params(TxParameters::new(None, Some(SCRIPT_GAS_LIMIT), None))
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(18446744073709486080)")]
    async fn when_input_is_zero() {
        expected_and_actual_output(SwapParameters {
            amount: 0,
            route_length: 2,
        })
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(18446744073709486080)")]
    // fails because starting with the second swap, the swap input is 0 which is not allowed
    async fn when_input_is_one_and_route_has_more_than_two_assets() {
        expected_and_actual_output(SwapParameters {
            amount: 1,
            route_length: NUMBER_OF_ASSETS,
        })
        .await;
    }
}
