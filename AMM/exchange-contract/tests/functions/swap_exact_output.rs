use crate::utils::setup_and_construct;
use test_utils::interface::exchange::{preview_swap_exact_output, swap_exact_output};

mod success {
    use super::*;
    use crate::utils::wallet_balances;
    use test_utils::interface::{exchange::pool_info, Asset, SwapEvent};

    #[tokio::test]
    async fn swaps_a_for_b_without_refund() {
        let (exchange, wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let output_amount = 10;

        let initial_pool_info = pool_info(&exchange.instance).await;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;

        let max_input =
            preview_swap_exact_output(&exchange.instance, output_amount, exchange.pair.1, true)
                .await
                .other_asset
                .amount;

        let response = swap_exact_output(
            &exchange.instance,
            exchange.pair.0,
            max_input,
            output_amount,
            liquidity_parameters.deadline,
            true,
        )
        .await;
        let log = response.decode_logs_with_type::<SwapEvent>().unwrap();
        let event = log.first().unwrap();

        let input_amount = response.value;

        let final_pool_info = pool_info(&exchange.instance).await;
        let final_wallet_balances = wallet_balances(&exchange, &wallet).await;

        assert_eq!(
            *event,
            SwapEvent {
                input: Asset {
                    id: exchange.pair.0,
                    amount: initial_pool_info.reserves.a.amount + max_input,
                },
                output: Asset {
                    id: exchange.pair.1,
                    amount: initial_pool_info.reserves.b.amount - output_amount,
                },
            }
        );
        assert!(input_amount <= max_input);
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
    async fn swaps_a_for_b_with_refund() {
        let (exchange, wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let output_amount = 10;
        let forward_extra = 100;

        let initial_pool_info = pool_info(&exchange.instance).await;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;

        let max_input =
            preview_swap_exact_output(&exchange.instance, output_amount, exchange.pair.1, true)
                .await
                .other_asset
                .amount;
        let forward_amount = max_input + forward_extra;

        let response = swap_exact_output(
            &exchange.instance,
            exchange.pair.0,
            forward_amount,
            output_amount,
            liquidity_parameters.deadline,
            true,
        )
        .await;
        let log = response.decode_logs_with_type::<SwapEvent>().unwrap();
        let event = log.first().unwrap();

        let input_amount = response.value;

        let final_pool_info = pool_info(&exchange.instance).await;
        let final_wallet_balances = wallet_balances(&exchange, &wallet).await;

        assert_eq!(
            *event,
            SwapEvent {
                input: Asset {
                    id: exchange.pair.0,
                    amount: initial_pool_info.reserves.a.amount + max_input,
                },
                output: Asset {
                    id: exchange.pair.1,
                    amount: initial_pool_info.reserves.b.amount - output_amount,
                },
            }
        );
        assert!(input_amount <= max_input);
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
    async fn swaps_b_for_a_without_refund() {
        let (exchange, wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let output_amount = 10;

        let initial_pool_info = pool_info(&exchange.instance).await;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;

        let max_input =
            preview_swap_exact_output(&exchange.instance, output_amount, exchange.pair.0, true)
                .await
                .other_asset
                .amount;

        let response = swap_exact_output(
            &exchange.instance,
            exchange.pair.1,
            max_input,
            output_amount,
            liquidity_parameters.deadline,
            true,
        )
        .await;
        let log = response.decode_logs_with_type::<SwapEvent>().unwrap();
        let event = log.first().unwrap();

        let input_amount = response.value;

        let final_pool_info = pool_info(&exchange.instance).await;
        let final_wallet_balances = wallet_balances(&exchange, &wallet).await;

        assert_eq!(
            *event,
            SwapEvent {
                input: Asset {
                    id: exchange.pair.1,
                    amount: initial_pool_info.reserves.b.amount + max_input,
                },
                output: Asset {
                    id: exchange.pair.0,
                    amount: initial_pool_info.reserves.a.amount - output_amount,
                },
            }
        );
        assert!(input_amount <= max_input);
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
    async fn swaps_b_for_a_with_refund() {
        let (exchange, wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let output_amount = 10;
        let forward_extra = 100;

        let initial_pool_info = pool_info(&exchange.instance).await;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;

        let max_input =
            preview_swap_exact_output(&exchange.instance, output_amount, exchange.pair.0, true)
                .await
                .other_asset
                .amount;
        let forward_amount = max_input + forward_extra;

        let response = swap_exact_output(
            &exchange.instance,
            exchange.pair.1,
            forward_amount,
            output_amount,
            liquidity_parameters.deadline,
            true,
        )
        .await;
        let log = response.decode_logs_with_type::<SwapEvent>().unwrap();
        let event = log.first().unwrap();

        let input_amount = response.value;

        let final_pool_info = pool_info(&exchange.instance).await;
        let final_wallet_balances = wallet_balances(&exchange, &wallet).await;

        assert_eq!(
            *event,
            SwapEvent {
                input: Asset {
                    id: exchange.pair.1,
                    amount: initial_pool_info.reserves.b.amount + max_input,
                },
                output: Asset {
                    id: exchange.pair.0,
                    amount: initial_pool_info.reserves.a.amount - output_amount,
                },
            }
        );
        assert!(input_amount <= max_input);
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
}

mod revert {
    use super::*;
    use crate::utils::setup;

    #[tokio::test]
    #[should_panic(expected = "AssetPairNotSet")]
    async fn when_uninitialized() {
        // call setup instead of setup_and_initialize
        let (exchange_instance, _wallet, assets, deadline) = setup().await;
        let output_amount = 10;

        swap_exact_output(
            &exchange_instance,
            assets.asset_1,
            1,
            output_amount,
            deadline,
            false,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidAsset")]
    async fn when_msg_asset_id_is_invalid() {
        let (exchange, _wallet, liquidity_parameters, asset_c_id) =
            setup_and_construct(true, true).await;

        let output_amount = 10;

        swap_exact_output(
            &exchange.instance,
            asset_c_id, // forwarding invalid asset
            1,
            output_amount,
            liquidity_parameters.deadline,
            false,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "DeadlinePassed")]
    async fn when_deadline_passed() {
        let (exchange, _wallet, _liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let output_amount = 10;

        swap_exact_output(
            &exchange.instance,
            exchange.pair.0,
            1,
            output_amount,
            0, // passing 0 deadline
            false,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "ExpectedNonZeroParameter")]
    async fn when_output_amount_is_zero() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        swap_exact_output(
            &exchange.instance,
            exchange.pair.0,
            1,
            0, // passing 0 amount
            liquidity_parameters.deadline,
            false,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "ExpectedNonZeroAmount")]
    async fn when_msg_amount_is_zero() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let output_amount = 10;

        swap_exact_output(
            &exchange.instance,
            exchange.pair.0,
            0, // forwarding 0 as msg_amount
            output_amount,
            liquidity_parameters.deadline,
            false,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "InsufficientReserve")]
    async fn when_output_amount_is_more_than_reserve() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let output_amount = 10;
        let forward_amount =
            preview_swap_exact_output(&exchange.instance, output_amount, exchange.pair.0, true)
                .await
                .other_asset
                .amount;

        swap_exact_output(
            &exchange.instance,
            exchange.pair.0,
            forward_amount,
            // requesting an output amount that is more than what the contract has in its reserves
            liquidity_parameters.amounts.0 + 1000000,
            liquidity_parameters.deadline,
            false,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "DesiredAmountTooHigh")]
    async fn when_forwarding_insufficient_amount_of_a() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let output_amount = 10;

        let preview_amount =
            preview_swap_exact_output(&exchange.instance, output_amount, exchange.pair.1, true)
                .await
                .other_asset
                .amount;

        // forwarding insufficient amount
        let forward_amount = preview_amount - 1;

        swap_exact_output(
            &exchange.instance,
            exchange.pair.0,
            forward_amount,
            output_amount,
            liquidity_parameters.deadline,
            true,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "DesiredAmountTooHigh")]
    async fn when_forwarding_insufficient_amount_of_b() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let output_amount = 10;

        let preview_amount =
            preview_swap_exact_output(&exchange.instance, output_amount, exchange.pair.0, true)
                .await
                .other_asset
                .amount;

        // forwarding insufficient amount
        let forward_amount = preview_amount - 1;

        swap_exact_output(
            &exchange.instance,
            exchange.pair.1,
            forward_amount,
            output_amount,
            liquidity_parameters.deadline,
            true,
        )
        .await;
    }
}
