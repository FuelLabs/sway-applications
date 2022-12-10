use crate::utils::{setup, setup_and_construct, wallet_balances};
use fuels::prelude::*;
use test_utils::abi::exchange::{pool_info, preview_swap_exact_input, swap_exact_input};

mod success {
    use super::*;

    #[tokio::test]
    async fn swaps_a_for_b() {
        let (exchange, wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let input_amount = 10;

        let initial_pool_info = pool_info(&exchange.instance).await.value;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;

        let min_output =
            preview_swap_exact_input(&exchange.instance, input_amount, exchange.pair.0)
                .await
                .value
                .amount;

        let output_amount = swap_exact_input(
            &exchange.instance,
            CallParameters::new(Some(input_amount), Some(exchange.pair.0), None),
            Some(min_output),
            liquidity_parameters.deadline,
        )
        .await
        .value;

        let final_pool_info = pool_info(&exchange.instance).await.value;
        let final_wallet_balances = wallet_balances(&exchange, &wallet).await;

        assert_eq!(output_amount >= min_output, true);
        assert_eq!(
            final_wallet_balances.asset_a,
            initial_wallet_balances.asset_a - input_amount
        );
        assert_eq!(
            final_wallet_balances.asset_b,
            initial_wallet_balances.asset_b + output_amount
        );
        assert_eq!(
            final_pool_info.asset_a_reserve,
            initial_pool_info.asset_a_reserve + input_amount
        );
        assert_eq!(
            final_pool_info.asset_b_reserve,
            initial_pool_info.asset_b_reserve - output_amount
        );
    }

    #[tokio::test]
    async fn swaps_b_for_a() {
        let (exchange, wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let input_amount = 10;

        let initial_pool_info = pool_info(&exchange.instance).await.value;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;

        let min_output =
            preview_swap_exact_input(&exchange.instance, input_amount, exchange.pair.1)
                .await
                .value
                .amount;

        let output_amount = swap_exact_input(
            &exchange.instance,
            CallParameters::new(Some(input_amount), Some(exchange.pair.1), None),
            Some(min_output),
            liquidity_parameters.deadline,
        )
        .await
        .value;

        let final_pool_info = pool_info(&exchange.instance).await.value;
        let final_wallet_balances = wallet_balances(&exchange, &wallet).await;

        assert_eq!(output_amount >= min_output, true);
        assert_eq!(
            final_wallet_balances.asset_b,
            initial_wallet_balances.asset_b - input_amount
        );
        assert_eq!(
            final_wallet_balances.asset_a,
            initial_wallet_balances.asset_a + output_amount
        );
        assert_eq!(
            final_pool_info.asset_b_reserve,
            initial_pool_info.asset_b_reserve + input_amount
        );
        assert_eq!(
            final_pool_info.asset_a_reserve,
            initial_pool_info.asset_a_reserve - output_amount
        );
    }

    #[tokio::test]
    async fn swaps_without_specifying_min_output() {
        let (exchange, wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let input_amount = 10;

        let initial_pool_info = pool_info(&exchange.instance).await.value;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;

        let output_amount = swap_exact_input(
            &exchange.instance,
            CallParameters::new(Some(input_amount), Some(exchange.pair.0), None),
            None,
            liquidity_parameters.deadline,
        )
        .await
        .value;

        let final_pool_info = pool_info(&exchange.instance).await.value;
        let final_wallet_balances = wallet_balances(&exchange, &wallet).await;

        assert_eq!(
            final_wallet_balances.asset_a,
            initial_wallet_balances.asset_a - input_amount
        );
        assert_eq!(
            final_wallet_balances.asset_b,
            initial_wallet_balances.asset_b + output_amount
        );
        assert_eq!(
            final_pool_info.asset_a_reserve,
            initial_pool_info.asset_a_reserve + input_amount
        );
        assert_eq!(
            final_pool_info.asset_b_reserve,
            initial_pool_info.asset_b_reserve - output_amount
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "AssetPairNotSet")]
    async fn when_uninitialized() {
        // call setup instead of setup_and_initialize
        let (exchange_instance, _wallet, _pool_asset_id, asset_a_id, _asset_b_id, _asset_c_id) =
            setup().await;

        let deadline = 1000;

        swap_exact_input(
            &exchange_instance,
            CallParameters::new(Some(1), Some(AssetId::new(*asset_a_id)), None),
            None,
            deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidAsset")]
    async fn when_msg_asset_id_is_invalid() {
        let (exchange, _wallet, liquidity_parameters, asset_c_id) =
            setup_and_construct(true, true).await;

        swap_exact_input(
            &exchange.instance,
            // forwarding invalid asset
            CallParameters::new(Some(1), Some(AssetId::new(*asset_c_id)), None),
            None,
            liquidity_parameters.deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "DeadlinePassed")]
    async fn when_deadline_has_passed() {
        let (exchange, _wallet, _liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        swap_exact_input(
            &exchange.instance,
            CallParameters::new(Some(1), Some(exchange.pair.0), None),
            None,
            // passing 0 deadline
            0,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "ExpectedNonZeroAmount")]
    async fn when_msg_amount_is_zero() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        swap_exact_input(
            &exchange.instance,
            // forwarding 0 as msg_amount
            CallParameters::new(Some(0), Some(exchange.pair.0), None),
            None,
            liquidity_parameters.deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "DesiredAmountTooHigh")]
    async fn when_minimum_a_constraint_is_too_high() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let input_amount = 10;

        let preview_amount =
            preview_swap_exact_input(&exchange.instance, input_amount, exchange.pair.0)
                .await
                .value
                .amount;

        swap_exact_input(
            &exchange.instance,
            CallParameters::new(Some(input_amount), Some(exchange.pair.0), None),
            // setting min too high
            Some(preview_amount + 1),
            liquidity_parameters.deadline,
        )
        .await;
    }
}
