use crate::utils::{setup, setup_and_construct, wallet_balances};
use fuels::prelude::*;
use test_utils::abi::exchange::{pool_info, preview_swap_exact_output, swap_exact_output};

mod success {
    use super::*;

    #[tokio::test]
    async fn swaps_a_for_b_without_refund() {
        let (exchange, wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let output_amount = 10;

        let initial_pool_info = pool_info(&exchange.instance).await.value;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;

        let max_input =
            preview_swap_exact_output(&exchange.instance, output_amount, exchange.pair.1)
                .await
                .value
                .amount;

        let input_amount = swap_exact_output(
            &exchange.instance,
            CallParameters::new(Some(max_input), Some(exchange.pair.0), Some(10_000_000)),
            output_amount,
            liquidity_parameters.deadline,
        )
        .await
        .value;

        let final_pool_info = pool_info(&exchange.instance).await.value;
        let final_wallet_balances = wallet_balances(&exchange, &wallet).await;

        assert_eq!(input_amount <= max_input, true);
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
    async fn swaps_a_for_b_with_refund() {
        let (exchange, wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let output_amount = 10;
        let forward_extra = 100;

        let initial_pool_info = pool_info(&exchange.instance).await.value;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;

        let max_input =
            preview_swap_exact_output(&exchange.instance, output_amount, exchange.pair.1)
                .await
                .value
                .amount;
        let forward_amount = max_input + forward_extra;

        let input_amount = swap_exact_output(
            &exchange.instance,
            CallParameters::new(
                Some(forward_amount),
                Some(exchange.pair.0),
                Some(10_000_000),
            ),
            output_amount,
            liquidity_parameters.deadline,
        )
        .await
        .value;

        let final_pool_info = pool_info(&exchange.instance).await.value;
        let final_wallet_balances = wallet_balances(&exchange, &wallet).await;

        assert_eq!(input_amount <= max_input, true);
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
    async fn swaps_b_for_a_without_refund() {
        let (exchange, wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let output_amount = 10;

        let initial_pool_info = pool_info(&exchange.instance).await.value;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;

        let max_input =
            preview_swap_exact_output(&exchange.instance, output_amount, exchange.pair.0)
                .await
                .value
                .amount;

        let input_amount = swap_exact_output(
            &exchange.instance,
            CallParameters::new(Some(max_input), Some(exchange.pair.1), Some(10_000_000)),
            output_amount,
            liquidity_parameters.deadline,
        )
        .await
        .value;

        let final_pool_info = pool_info(&exchange.instance).await.value;
        let final_wallet_balances = wallet_balances(&exchange, &wallet).await;

        assert_eq!(input_amount <= max_input, true);
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
    async fn swaps_b_for_a_with_refund() {
        let (exchange, wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let output_amount = 10;
        let forward_extra = 100;

        let initial_pool_info = pool_info(&exchange.instance).await.value;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;

        let max_input =
            preview_swap_exact_output(&exchange.instance, output_amount, exchange.pair.0)
                .await
                .value
                .amount;
        let forward_amount = max_input + forward_extra;

        let input_amount = swap_exact_output(
            &exchange.instance,
            CallParameters::new(
                Some(forward_amount),
                Some(exchange.pair.1),
                Some(10_000_000),
            ),
            output_amount,
            liquidity_parameters.deadline,
        )
        .await
        .value;

        let final_pool_info = pool_info(&exchange.instance).await.value;
        let final_wallet_balances = wallet_balances(&exchange, &wallet).await;

        assert_eq!(input_amount <= max_input, true);
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
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "RevertTransactionError(\"NotInitialized\"")]
    async fn when_unitialized() {
        // call setup instead of setup_and_initialize
        let (exchange_instance, _wallet, _pool_asset_id, asset_a_id, _asset_b_id, _asset_c_id) =
            setup().await;
        let output_amount = 10;
        let deadline = 1000;

        swap_exact_output(
            &exchange_instance,
            CallParameters::new(Some(1), Some(AssetId::new(*asset_a_id)), Some(10_000_000)),
            output_amount,
            deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "RevertTransactionError(\"InvalidAsset\"")]
    async fn when_msg_asset_id_is_invalid() {
        let (exchange, _wallet, liquidity_parameters, asset_c_id) =
            setup_and_construct(true, true).await;

        let output_amount = 10;

        swap_exact_output(
            &exchange.instance,
            // sending invalid asset
            CallParameters::new(Some(1), Some(AssetId::new(*asset_c_id)), Some(10_000_000)),
            output_amount,
            liquidity_parameters.deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "RevertTransactionError(\"AmountCannotBeZero\"")]
    async fn when_output_amount_is_zero() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        swap_exact_output(
            &exchange.instance,
            CallParameters::new(Some(1), Some(exchange.pair.0), Some(10_000_000)),
            // passing 0 amount
            0,
            liquidity_parameters.deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "RevertTransactionError(\"DeadlinePassed(0)\"")]
    async fn when_deadline_has_passed() {
        let (exchange, _wallet, _liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let output_amount = 10;

        swap_exact_output(
            &exchange.instance,
            CallParameters::new(Some(1), Some(exchange.pair.0), Some(10_000_000)),
            output_amount,
            // passing 0 deadline
            0,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "RevertTransactionError(\"AmountCannotBeZero\"")]
    async fn when_msg_amount_is_zero() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let output_amount = 10;

        swap_exact_output(
            &exchange.instance,
            // forwarding 0 as msg_amount
            CallParameters::new(Some(0), Some(exchange.pair.0), Some(10_000_000)),
            output_amount,
            liquidity_parameters.deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "RevertTransactionError(\"ProvidedAmountTooLow(2)\"")]
    async fn when_forwarding_insufficient_amount_of_a() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let output_amount = 10;

        let preview_amount =
            preview_swap_exact_output(&exchange.instance, output_amount, exchange.pair.1)
                .await
                .value
                .amount;
        // forwarding insufficient amount
        let forward_amount = preview_amount - 1;

        swap_exact_output(
            &exchange.instance,
            CallParameters::new(
                Some(forward_amount),
                Some(exchange.pair.0),
                Some(10_000_000),
            ),
            output_amount,
            liquidity_parameters.deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "RevertTransactionError(\"ProvidedAmountTooLow(44)\"")]
    async fn when_forwarding_insufficient_amount_of_b() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let output_amount = 10;

        let preview_amount =
            preview_swap_exact_output(&exchange.instance, output_amount, exchange.pair.0)
                .await
                .value
                .amount;
        // forwarding insufficient amount
        let forward_amount = preview_amount - 1;

        swap_exact_output(
            &exchange.instance,
            CallParameters::new(
                Some(forward_amount),
                Some(exchange.pair.1),
                Some(10_000_000),
            ),
            output_amount,
            liquidity_parameters.deadline,
        )
        .await;
    }
}
