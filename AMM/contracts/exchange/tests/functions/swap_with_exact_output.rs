use crate::utils::{
    abi_calls::{pool_info, preview_swap_with_exact_output, swap_with_exact_output},
    test_helpers::{deposit_and_add_liquidity, setup, setup_and_initialize},
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn swaps_a_for_b_without_refund() {
        let (exchange, wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 400;
        let liquidity = 200;
        let deadline = 1000;
        let output_amount = 10;

        deposit_and_add_liquidity(
            &exchange.contract,
            exchange.asset_a_asset_id,
            deposit_amount_a,
            exchange.asset_b_asset_id,
            deposit_amount_b,
            liquidity,
            deadline,
        )
        .await;

        let wallet_initial_balance_a = wallet
            .get_asset_balance(&exchange.asset_a_asset_id)
            .await
            .unwrap();
        let wallet_initial_balance_b = wallet
            .get_asset_balance(&exchange.asset_b_asset_id)
            .await
            .unwrap();
        let initial_pool_info = pool_info(&exchange.contract).await.value;

        let max_input = preview_swap_with_exact_output(
            &exchange.contract,
            output_amount,
            exchange.asset_b_contract_id,
        )
        .await
        .value
        .amount;

        let input_amount = swap_with_exact_output(
            &exchange.contract,
            CallParameters::new(Some(max_input), Some(exchange.asset_a_asset_id), None),
            output_amount,
            deadline,
        )
        .await
        .value;

        let wallet_final_balance_a = wallet
            .get_asset_balance(&exchange.asset_a_asset_id)
            .await
            .unwrap();
        let wallet_final_balance_b = wallet
            .get_asset_balance(&exchange.asset_b_asset_id)
            .await
            .unwrap();
        let final_pool_info = pool_info(&exchange.contract).await.value;

        assert_eq!(input_amount <= max_input, true);
        assert_eq!(
            wallet_final_balance_a,
            wallet_initial_balance_a - input_amount
        );
        assert_eq!(
            wallet_final_balance_b,
            wallet_initial_balance_b + output_amount
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
        let (exchange, wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 400;
        let liquidity = 200;
        let deadline = 1000;
        let output_amount = 10;
        let forward_extra = 100;

        deposit_and_add_liquidity(
            &exchange.contract,
            exchange.asset_a_asset_id,
            deposit_amount_a,
            exchange.asset_b_asset_id,
            deposit_amount_b,
            liquidity,
            deadline,
        )
        .await;

        let wallet_initial_balance_a = wallet
            .get_asset_balance(&exchange.asset_a_asset_id)
            .await
            .unwrap();
        let wallet_initial_balance_b = wallet
            .get_asset_balance(&exchange.asset_b_asset_id)
            .await
            .unwrap();
        let initial_pool_info = pool_info(&exchange.contract).await.value;

        let max_input = preview_swap_with_exact_output(
            &exchange.contract,
            output_amount,
            exchange.asset_b_contract_id,
        )
        .await
        .value
        .amount;
        let forward_amount = max_input + forward_extra;

        let input_amount = swap_with_exact_output(
            &exchange.contract,
            CallParameters::new(Some(forward_amount), Some(exchange.asset_a_asset_id), None),
            output_amount,
            deadline,
        )
        .await
        .value;

        let wallet_final_balance_a = wallet
            .get_asset_balance(&exchange.asset_a_asset_id)
            .await
            .unwrap();
        let wallet_final_balance_b = wallet
            .get_asset_balance(&exchange.asset_b_asset_id)
            .await
            .unwrap();
        let final_pool_info = pool_info(&exchange.contract).await.value;

        assert_eq!(input_amount <= max_input, true);
        assert_eq!(
            wallet_final_balance_a,
            wallet_initial_balance_a - input_amount
        );
        assert_eq!(
            wallet_final_balance_b,
            wallet_initial_balance_b + output_amount
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
        let (exchange, wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 400;
        let liquidity = 200;
        let deadline = 1000;
        let output_amount = 10;

        deposit_and_add_liquidity(
            &exchange.contract,
            exchange.asset_a_asset_id,
            deposit_amount_a,
            exchange.asset_b_asset_id,
            deposit_amount_b,
            liquidity,
            deadline,
        )
        .await;

        let wallet_initial_balance_a = wallet
            .get_asset_balance(&exchange.asset_a_asset_id)
            .await
            .unwrap();
        let wallet_initial_balance_b = wallet
            .get_asset_balance(&exchange.asset_b_asset_id)
            .await
            .unwrap();
        let initial_pool_info = pool_info(&exchange.contract).await.value;

        let max_input = preview_swap_with_exact_output(
            &exchange.contract,
            output_amount,
            exchange.asset_a_contract_id,
        )
        .await
        .value
        .amount;

        let input_amount = swap_with_exact_output(
            &exchange.contract,
            CallParameters::new(Some(max_input), Some(exchange.asset_b_asset_id), None),
            output_amount,
            deadline,
        )
        .await
        .value;

        let wallet_final_balance_a = wallet
            .get_asset_balance(&exchange.asset_a_asset_id)
            .await
            .unwrap();
        let wallet_final_balance_b = wallet
            .get_asset_balance(&exchange.asset_b_asset_id)
            .await
            .unwrap();
        let final_pool_info = pool_info(&exchange.contract).await.value;

        assert_eq!(input_amount <= max_input, true);
        assert_eq!(
            wallet_final_balance_b,
            wallet_initial_balance_b - input_amount
        );
        assert_eq!(
            wallet_final_balance_a,
            wallet_initial_balance_a + output_amount
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
        let (exchange, wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 400;
        let liquidity = 200;
        let deadline = 1000;
        let output_amount = 10;
        let forward_extra = 100;

        deposit_and_add_liquidity(
            &exchange.contract,
            exchange.asset_a_asset_id,
            deposit_amount_a,
            exchange.asset_b_asset_id,
            deposit_amount_b,
            liquidity,
            deadline,
        )
        .await;

        let wallet_initial_balance_a = wallet
            .get_asset_balance(&exchange.asset_a_asset_id)
            .await
            .unwrap();
        let wallet_initial_balance_b = wallet
            .get_asset_balance(&exchange.asset_b_asset_id)
            .await
            .unwrap();
        let initial_pool_info = pool_info(&exchange.contract).await.value;

        let max_input = preview_swap_with_exact_output(
            &exchange.contract,
            output_amount,
            exchange.asset_a_contract_id,
        )
        .await
        .value
        .amount;
        let forward_amount = max_input + forward_extra;

        let input_amount = swap_with_exact_output(
            &exchange.contract,
            CallParameters::new(Some(forward_amount), Some(exchange.asset_b_asset_id), None),
            output_amount,
            deadline,
        )
        .await
        .value;

        let wallet_final_balance_a = wallet
            .get_asset_balance(&exchange.asset_a_asset_id)
            .await
            .unwrap();
        let wallet_final_balance_b = wallet
            .get_asset_balance(&exchange.asset_b_asset_id)
            .await
            .unwrap();
        let final_pool_info = pool_info(&exchange.contract).await.value;

        assert_eq!(input_amount <= max_input, true);
        assert_eq!(
            wallet_final_balance_b,
            wallet_initial_balance_b - input_amount
        );
        assert_eq!(
            wallet_final_balance_a,
            wallet_initial_balance_a + output_amount
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
    #[should_panic(expected = "Revert(42)")]
    async fn when_unitialized() {
        // call setup instead of setup_and_initialize
        let (exchange_instance, _wallet, _pool_asset_id, asset_a_id, _asset_b_id, _asset_c_id) =
            setup().await;
        let output_amount = 10;
        let deadline = 1000;

        swap_with_exact_output(
            &exchange_instance,
            CallParameters::new(Some(1), Some(AssetId::new(*asset_a_id)), None),
            output_amount,
            deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_msg_asset_id_is_invalid() {
        let (exchange, _wallet, asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 400;
        let liquidity = 200;
        let deadline = 1000;
        let output_amount = 10;

        deposit_and_add_liquidity(
            &exchange.contract,
            exchange.asset_a_asset_id,
            deposit_amount_a,
            exchange.asset_b_asset_id,
            deposit_amount_b,
            liquidity,
            deadline,
        )
        .await;

        swap_with_exact_output(
            &exchange.contract,
            // sending invalid asset
            CallParameters::new(Some(1), Some(AssetId::new(*asset_c_id)), None),
            output_amount,
            deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_output_amount_is_zero() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 400;
        let liquidity = 200;
        let deadline = 1000;

        deposit_and_add_liquidity(
            &exchange.contract,
            exchange.asset_a_asset_id,
            deposit_amount_a,
            exchange.asset_b_asset_id,
            deposit_amount_b,
            liquidity,
            deadline,
        )
        .await;

        swap_with_exact_output(
            &exchange.contract,
            CallParameters::new(Some(1), Some(exchange.asset_a_asset_id), None),
            // passing 0 amount
            0,
            deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_deadline_has_passed() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 400;
        let liquidity = 200;
        let deadline = 1000;
        let output_amount = 10;

        deposit_and_add_liquidity(
            &exchange.contract,
            exchange.asset_a_asset_id,
            deposit_amount_a,
            exchange.asset_b_asset_id,
            deposit_amount_b,
            liquidity,
            deadline,
        )
        .await;

        swap_with_exact_output(
            &exchange.contract,
            CallParameters::new(Some(1), Some(exchange.asset_a_asset_id), None),
            output_amount,
            // passing 0 deadline
            0,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_msg_amount_is_zero() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 400;
        let liquidity = 200;
        let deadline = 1000;
        let output_amount = 10;

        deposit_and_add_liquidity(
            &exchange.contract,
            exchange.asset_a_asset_id,
            deposit_amount_a,
            exchange.asset_b_asset_id,
            deposit_amount_b,
            liquidity,
            deadline,
        )
        .await;

        swap_with_exact_output(
            &exchange.contract,
            // forwarding 0 as msg_amount
            CallParameters::new(Some(0), Some(exchange.asset_a_asset_id), None),
            output_amount,
            deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_forwarding_insufficient_amount_of_a() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 400;
        let liquidity = 200;
        let deadline = 1000;
        let output_amount = 10;

        deposit_and_add_liquidity(
            &exchange.contract,
            exchange.asset_a_asset_id,
            deposit_amount_a,
            exchange.asset_b_asset_id,
            deposit_amount_b,
            liquidity,
            deadline,
        )
        .await;

        let preview_amount = preview_swap_with_exact_output(
            &exchange.contract,
            output_amount,
            exchange.asset_b_contract_id,
        )
        .await
        .value
        .amount;
        // forwarding insufficient amount
        let forward_amount = preview_amount - 1;

        swap_with_exact_output(
            &exchange.contract,
            CallParameters::new(Some(forward_amount), Some(exchange.asset_a_asset_id), None),
            output_amount,
            deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_forwarding_insufficient_amount_of_b() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 400;
        let liquidity = 200;
        let deadline = 1000;
        let output_amount = 10;

        deposit_and_add_liquidity(
            &exchange.contract,
            exchange.asset_a_asset_id,
            deposit_amount_a,
            exchange.asset_b_asset_id,
            deposit_amount_b,
            liquidity,
            deadline,
        )
        .await;

        let preview_amount = preview_swap_with_exact_output(
            &exchange.contract,
            output_amount,
            exchange.asset_a_contract_id,
        )
        .await
        .value
        .amount;
        // forwarding insufficient amount
        let forward_amount = preview_amount - 1;

        swap_with_exact_output(
            &exchange.contract,
            CallParameters::new(Some(forward_amount), Some(exchange.asset_b_asset_id), None),
            output_amount,
            deadline,
        )
        .await;
    }
}
