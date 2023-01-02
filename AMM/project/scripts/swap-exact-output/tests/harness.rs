use fuels::prelude::*;
use test_utils::{
    data_structures::{AMMContract, TransactionParameters, WalletAssetConfiguration},
    interface::{exchange::preview_swap_exact_output, SwapExactOutputScript, SCRIPT_GAS_LIMIT},
    paths::SWAP_EXACT_OUTPUT_SCRIPT_BINARY_PATH,
    setup::{
        common::{deploy_and_initialize_amm, setup_wallet_and_provider},
        scripts::{setup_exchange_contracts, transaction_inputs_outputs},
    },
};

async fn expected_swap_input(amm: &AMMContract, output_amount: u64, route: &Vec<AssetId>) -> u64 {
    assert!(route.len() >= 2);
    let (mut i, mut latest_input) = (route.len() - 1, output_amount);

    while i > 0 {
        let pair = (*route.get(i - 1).unwrap(), *route.get(i).unwrap());
        let exchange = &amm.pools.get(&pair).unwrap().instance;
        latest_input = preview_swap_exact_output(&exchange, latest_input, pair.1, true)
            .await
            .other_asset
            .amount;
        i -= 1;
    }
    latest_input
}

async fn setup() -> (
    SwapExactOutputScript,
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

    let script_instance = SwapExactOutputScript::new(wallet, SWAP_EXACT_OUTPUT_SCRIPT_BINARY_PATH);

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
    async fn can_swap_exact_output_along_route() {
        let (script_instance, amm, asset_ids, transaction_parameters, deadline) = setup().await;

        let route = asset_ids;
        let output_amount = 10_000;

        let expected_result = expected_swap_input(&amm, output_amount, &route).await;

        let result = script_instance
            .main(
                amm.id,
                route
                    .into_iter()
                    .map(|asset_id| ContractId::new(*asset_id))
                    .collect(),
                output_amount,
                expected_result,
                deadline,
            )
            .with_inputs(transaction_parameters.inputs)
            .with_outputs(transaction_parameters.outputs)
            .tx_params(TxParameters::new(None, Some(SCRIPT_GAS_LIMIT), None))
            .call()
            .await
            .unwrap()
            .value;

        assert_eq!(expected_result, result);
    }

    #[tokio::test]
    async fn can_swap_exact_output_two_assets() {
        let (script_instance, amm, asset_ids, transaction_parameters, deadline) = setup().await;

        // route consists of two assets. this is a direct swap
        let route = vec![*asset_ids.get(0).unwrap(), *asset_ids.get(1).unwrap()];
        let output_amount = 10_000;

        let expected_result = expected_swap_input(&amm, output_amount, &route).await;

        let result = script_instance
            .main(
                amm.id,
                route
                    .into_iter()
                    .map(|asset_id| ContractId::new(*asset_id))
                    .collect(),
                output_amount,
                expected_result,
                deadline,
            )
            .with_inputs(transaction_parameters.inputs)
            .with_outputs(transaction_parameters.outputs)
            .tx_params(TxParameters::new(None, Some(SCRIPT_GAS_LIMIT), None))
            .call()
            .await
            .unwrap()
            .value;

        assert_eq!(expected_result, result);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "RouteTooShort")]
    async fn when_route_length_is_zero() {
        let (script_instance, amm, _asset_ids, _transaction_parameters, deadline) = setup().await;

        // route length is zero
        let route: Vec<AssetId> = vec![];
        let output_amount = 0;
        let maximum_input_amount = 0;

        script_instance
            .main(
                amm.id,
                route
                    .into_iter()
                    .map(|asset_id| ContractId::new(*asset_id))
                    .collect(),
                output_amount,
                maximum_input_amount,
                deadline,
            )
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "RouteTooShort")]
    async fn when_route_length_is_one() {
        let (script_instance, amm, asset_ids, _transaction_parameters, deadline) = setup().await;

        // route length is one
        let route: Vec<AssetId> = vec![*asset_ids.get(0).unwrap()];
        let output_amount = 60;
        let maximum_input_amount = 0;

        script_instance
            .main(
                amm.id,
                route
                    .into_iter()
                    .map(|asset_id| ContractId::new(*asset_id))
                    .collect(),
                output_amount,
                maximum_input_amount,
                deadline,
            )
            .call()
            .await
            .unwrap();
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
            .main(
                amm.id,
                route
                    .into_iter()
                    .map(|asset_id| ContractId::new(*asset_id))
                    .collect(),
                output_amount,
                maximum_input_amount,
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
        let output_amount = 10_000;
        let maximum_input_amount = expected_swap_input(&amm, output_amount, &route).await;

        script_instance
            .main(
                amm.id,
                route
                    .into_iter()
                    .map(|asset_id| ContractId::new(*asset_id))
                    .collect(),
                output_amount,
                maximum_input_amount,
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
    async fn when_maximum_input_not_satisfied() {
        let (script_instance, amm, asset_ids, transaction_parameters, deadline) = setup().await;

        let route = asset_ids;
        let output_amount = 10_000;
        let maximum_input_amount = expected_swap_input(&amm, output_amount, &route).await;

        script_instance
            .main(
                amm.id,
                route
                    .into_iter()
                    .map(|asset_id| ContractId::new(*asset_id))
                    .collect(),
                output_amount,
                maximum_input_amount - 1, // setting the maximum to be lower than what it can be
                deadline,
            )
            .with_inputs(transaction_parameters.inputs)
            .with_outputs(transaction_parameters.outputs)
            .tx_params(TxParameters::new(None, Some(SCRIPT_GAS_LIMIT), None))
            .call()
            .await
            .unwrap();
    }
}
