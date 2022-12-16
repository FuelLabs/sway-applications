use fuels::prelude::*;
use test_utils::{
    abi::{exchange::preview_swap_exact_input, SwapExactInputScript, SCRIPT_GAS_LIMIT},
    data_structures::{AMMContract, TransactionParameters, WalletAssetConfiguration},
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
            .amount;
        i += 1;
    }
    latest_output
}

async fn setup() -> (
    WalletUnlocked,
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

    (wallet, amm, asset_ids, transaction_parameters, deadline)
}

mod success {
    use super::*;

    #[tokio::test]
    async fn can_swap_exact_input_along_route() {
        let (wallet, amm, asset_ids, transaction_parameters, deadline) = setup().await;

        let route = asset_ids;
        let script_instance =
            SwapExactInputScript::new(wallet, SWAP_EXACT_INPUT_SCRIPT_BINARY_PATH);
        let input_amount: u64 = 60;

        let expected_result = expected_swap_output(&amm, input_amount, &route).await;

        let result = script_instance
            .main(
                amm.id,
                route
                    .into_iter()
                    .map(|asset_id| ContractId::new(*asset_id))
                    .collect(),
                input_amount,
                Some(expected_result),
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
    async fn can_swap_exact_input_two_assets() {
        let (wallet, amm, asset_ids, transaction_parameters, deadline) = setup().await;

        // route consists of two assets. this is a direct swap
        let route = vec![*asset_ids.get(0).unwrap(), *asset_ids.get(1).unwrap()];
        let script_instance =
            SwapExactInputScript::new(wallet, SWAP_EXACT_INPUT_SCRIPT_BINARY_PATH);
        let input_amount: u64 = 60;

        let expected_result = expected_swap_output(&amm, input_amount, &route).await;

        let result = script_instance
            .main(
                amm.id,
                route
                    .into_iter()
                    .map(|asset_id| ContractId::new(*asset_id))
                    .collect(),
                input_amount,
                Some(expected_result),
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
        let (wallet, amm, _asset_ids, _transaction_parameters, deadline) = setup().await;

        // route length is zero
        let route: Vec<AssetId> = vec![];
        let input_amount = 10;
        let script_instance =
            SwapExactInputScript::new(wallet, SWAP_EXACT_INPUT_SCRIPT_BINARY_PATH);

        script_instance
            .main(
                amm.id,
                route
                    .into_iter()
                    .map(|asset_id| ContractId::new(*asset_id))
                    .collect(),
                input_amount,
                None,
                deadline,
            )
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "RouteTooShort")]
    async fn when_route_length_is_one() {
        let (wallet, amm, asset_ids, _transaction_parameters, deadline) = setup().await;

        // route length is one
        let route: Vec<AssetId> = vec![*asset_ids.get(0).unwrap()];
        let input_amount = 10;
        let script_instance =
            SwapExactInputScript::new(wallet, SWAP_EXACT_INPUT_SCRIPT_BINARY_PATH);

        script_instance
            .main(
                amm.id,
                route
                    .into_iter()
                    .map(|asset_id| ContractId::new(*asset_id))
                    .collect(),
                input_amount,
                None,
                deadline,
            )
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "PairExchangeNotRegistered")]
    async fn when_pair_exchange_not_registered() {
        let (wallet, amm, asset_ids, transaction_parameters, deadline) = setup().await;

        let mut route = asset_ids;

        let script_instance =
            SwapExactInputScript::new(wallet, SWAP_EXACT_INPUT_SCRIPT_BINARY_PATH);
        let input_amount: u64 = 60;

        // make sure that the first asset in the route does not have a pool
        let not_registered_asset_id = AssetId::from([1u8; 32]);
        route.remove(0);
        route.insert(0, not_registered_asset_id);

        script_instance
            .main(
                amm.id,
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
        let (wallet, amm, asset_ids, transaction_parameters, _deadline) = setup().await;

        let route = asset_ids;
        let script_instance =
            SwapExactInputScript::new(wallet, SWAP_EXACT_INPUT_SCRIPT_BINARY_PATH);
        let input_amount: u64 = 60;

        let expected_result = expected_swap_output(&amm, input_amount, &route).await;

        script_instance
            .main(
                amm.id,
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
        let (wallet, amm, asset_ids, transaction_parameters, deadline) = setup().await;

        let route = asset_ids;
        let script_instance =
            SwapExactInputScript::new(wallet, SWAP_EXACT_INPUT_SCRIPT_BINARY_PATH);
        let input_amount: u64 = 60;

        let expected_result = expected_swap_output(&amm, input_amount, &route).await;

        script_instance
            .main(
                amm.id,
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
}
