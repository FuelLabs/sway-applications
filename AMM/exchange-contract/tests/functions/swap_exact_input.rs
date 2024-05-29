use crate::utils::setup_and_construct;
use test_utils::interface::exchange::{preview_swap_exact_input, swap_exact_input};

mod success {
    use super::*;
    use crate::utils::wallet_balances;
    use test_utils::interface::{exchange::pool_info, Asset, SwapEvent};

    #[tokio::test]
    async fn swaps_a_for_b() {
        let (exchange, wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let input_amount = 10;

        let initial_pool_info = pool_info(&exchange.instance).await;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;

        let min_output =
            preview_swap_exact_input(&exchange.instance, input_amount, exchange.pair.0, true)
                .await
                .other_asset
                .amount;

        let response = swap_exact_input(
            &exchange.instance,
            exchange.pair.0,
            input_amount,
            Some(min_output),
            liquidity_parameters.deadline,
            true,
        )
        .await;
        let log = response.decode_logs_with_type::<SwapEvent>().unwrap();
        let event = log.first().unwrap();

        let output_amount = response.value;

        let final_pool_info = pool_info(&exchange.instance).await;
        let final_wallet_balances = wallet_balances(&exchange, &wallet).await;

        assert_eq!(
            *event,
            SwapEvent {
                input: Asset {
                    id: exchange.pair.0,
                    amount: initial_pool_info.reserves.a.amount + input_amount,
                },
                output: Asset {
                    id: exchange.pair.1,
                    amount: initial_pool_info.reserves.b.amount - min_output,
                },
            }
        );
        assert!(output_amount >= min_output);
        assert_eq!(
            final_wallet_balances.asset_a,
            initial_wallet_balances.asset_a - input_amount
        );
        assert_eq!(
            final_wallet_balances.asset_b,
            initial_wallet_balances.asset_b + output_amount
        );
        assert_eq!(
            final_pool_info.reserves.a.amount,
            initial_pool_info.reserves.a.amount + input_amount
        );
        assert_eq!(
            final_pool_info.reserves.b.amount,
            initial_pool_info.reserves.b.amount - output_amount
        );
    }

    #[tokio::test]
    async fn swaps_b_for_a() {
        let (exchange, wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let input_amount = 10;

        let initial_pool_info = pool_info(&exchange.instance).await;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;

        let min_output =
            preview_swap_exact_input(&exchange.instance, input_amount, exchange.pair.1, true)
                .await
                .other_asset
                .amount;

        let response = swap_exact_input(
            &exchange.instance,
            exchange.pair.1,
            input_amount,
            Some(min_output),
            liquidity_parameters.deadline,
            true,
        )
        .await;
        let log = response.decode_logs_with_type::<SwapEvent>().unwrap();
        let event = log.first().unwrap();

        let output_amount = response.value;

        let final_pool_info = pool_info(&exchange.instance).await;
        let final_wallet_balances = wallet_balances(&exchange, &wallet).await;

        assert_eq!(
            *event,
            SwapEvent {
                input: Asset {
                    id: exchange.pair.1,
                    amount: initial_pool_info.reserves.b.amount + input_amount,
                },
                output: Asset {
                    id: exchange.pair.0,
                    amount: initial_pool_info.reserves.a.amount - min_output,
                },
            }
        );
        assert!(output_amount >= min_output);
        assert_eq!(
            final_wallet_balances.asset_b,
            initial_wallet_balances.asset_b - input_amount
        );
        assert_eq!(
            final_wallet_balances.asset_a,
            initial_wallet_balances.asset_a + output_amount
        );
        assert_eq!(
            final_pool_info.reserves.b.amount,
            initial_pool_info.reserves.b.amount + input_amount
        );
        assert_eq!(
            final_pool_info.reserves.a.amount,
            initial_pool_info.reserves.a.amount - output_amount
        );
    }

    #[tokio::test]
    async fn swaps_without_specifying_min_output() {
        let (exchange, wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let input_amount = 10;

        let initial_pool_info = pool_info(&exchange.instance).await;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;

        let min_output =
            preview_swap_exact_input(&exchange.instance, input_amount, exchange.pair.0, true)
                .await
                .other_asset
                .amount;

        let response = swap_exact_input(
            &exchange.instance,
            exchange.pair.0,
            input_amount,
            None,
            liquidity_parameters.deadline,
            true,
        )
        .await;
        let log = response.decode_logs_with_type::<SwapEvent>().unwrap();
        let event = log.first().unwrap();

        let output_amount = response.value;

        let final_pool_info = pool_info(&exchange.instance).await;
        let final_wallet_balances = wallet_balances(&exchange, &wallet).await;

        assert_eq!(
            *event,
            SwapEvent {
                input: Asset {
                    id: exchange.pair.0,
                    amount: initial_pool_info.reserves.a.amount + input_amount,
                },
                output: Asset {
                    id: exchange.pair.1,
                    amount: initial_pool_info.reserves.b.amount - min_output,
                },
            }
        );
        assert_eq!(
            final_wallet_balances.asset_a,
            initial_wallet_balances.asset_a - input_amount
        );
        assert_eq!(
            final_wallet_balances.asset_b,
            initial_wallet_balances.asset_b + output_amount
        );
        assert_eq!(
            final_pool_info.reserves.a.amount,
            initial_pool_info.reserves.a.amount + input_amount
        );
        assert_eq!(
            final_pool_info.reserves.b.amount,
            initial_pool_info.reserves.b.amount - output_amount
        );
    }
}

mod revert {
    use super::*;
    use crate::utils::setup;

    #[tokio::test]
    #[should_panic(expected = "AssetPairNotSet")]
    async fn when_uninitialized() {
        // call setup instead of setup_and_initialize
        let (exchange_instance, _wallet, assets, deadline) = setup().await;

        swap_exact_input(&exchange_instance, assets.asset_1, 1, None, deadline, false).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidAsset")]
    async fn when_msg_asset_id_is_invalid() {
        let (exchange, _wallet, liquidity_parameters, asset_c_id) =
            setup_and_construct(true, true).await;

        swap_exact_input(
            &exchange.instance,
            asset_c_id,
            1,
            None,
            liquidity_parameters.deadline,
            false,
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
            exchange.pair.0,
            1,
            None,
            0, // passing 0 deadline
            false,
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
            exchange.pair.0,
            0, // forwarding 0 as msg_amount
            None,
            liquidity_parameters.deadline,
            false,
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
            preview_swap_exact_input(&exchange.instance, input_amount, exchange.pair.0, true)
                .await
                .other_asset
                .amount;

        swap_exact_input(
            &exchange.instance,
            exchange.pair.0,
            input_amount,
            Some(preview_amount + 1), // setting min too high
            liquidity_parameters.deadline,
            true,
        )
        .await;
    }
}
