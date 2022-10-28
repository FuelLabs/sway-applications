use crate::utils::{
    abi_calls::{pool_info, preview_swap_with_exact_input, swap_with_exact_input},
    test_helpers::{deposit_and_add_liquidity, setup, setup_and_initialize},
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn swaps_a_for_b() {
        let (exchange, wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 400;
        let liquidity = 200;
        let deadline = 1000;
        let input_amount = 10;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            liquidity,
            deadline,
        )
        .await;

        let wallet_initial_balance_a = wallet
            .get_asset_balance(&AssetId::new(*exchange.asset_a_id))
            .await
            .unwrap();
        let wallet_initial_balance_b = wallet
            .get_asset_balance(&AssetId::new(*exchange.asset_b_id))
            .await
            .unwrap();
        let initial_pool_info = pool_info(&exchange.contract).await.value;

        let min_output =
            preview_swap_with_exact_input(&exchange.contract, input_amount, exchange.asset_a_id)
                .await
                .value
                .amount;

        let output_amount = swap_with_exact_input(
            &exchange.contract,
            CallParameters::new(
                Some(input_amount),
                Some(AssetId::new(*exchange.asset_a_id)),
                None,
            ),
            Some(min_output),
            deadline,
        )
        .await
        .value;

        let wallet_final_balance_a = wallet
            .get_asset_balance(&AssetId::new(*exchange.asset_a_id))
            .await
            .unwrap();
        let wallet_final_balance_b = wallet
            .get_asset_balance(&AssetId::new(*exchange.asset_b_id))
            .await
            .unwrap();
        let final_pool_info = pool_info(&exchange.contract).await.value;

        assert_eq!(output_amount >= min_output, true);
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
    async fn swaps_b_for_a() {
        let (exchange, wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 400;
        let liquidity = 200;
        let deadline = 1000;
        let input_amount = 10;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            liquidity,
            deadline,
        )
        .await;

        let wallet_initial_balance_a = wallet
            .get_asset_balance(&AssetId::new(*exchange.asset_a_id))
            .await
            .unwrap();
        let wallet_initial_balance_b = wallet
            .get_asset_balance(&AssetId::new(*exchange.asset_b_id))
            .await
            .unwrap();
        let initial_pool_info = pool_info(&exchange.contract).await.value;

        let min_output =
            preview_swap_with_exact_input(&exchange.contract, input_amount, exchange.asset_b_id)
                .await
                .value
                .amount;

        let output_amount = swap_with_exact_input(
            &exchange.contract,
            CallParameters::new(
                Some(input_amount),
                Some(AssetId::new(*exchange.asset_b_id)),
                None,
            ),
            Some(min_output),
            deadline,
        )
        .await
        .value;

        let wallet_final_balance_a = wallet
            .get_asset_balance(&AssetId::new(*exchange.asset_a_id))
            .await
            .unwrap();
        let wallet_final_balance_b = wallet
            .get_asset_balance(&AssetId::new(*exchange.asset_b_id))
            .await
            .unwrap();
        let final_pool_info = pool_info(&exchange.contract).await.value;

        assert_eq!(output_amount >= min_output, true);
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
    async fn swaps_without_specifying_min_output() {
        let (exchange, wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 400;
        let liquidity = 200;
        let deadline = 1000;
        let input_amount = 10;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            liquidity,
            deadline,
        )
        .await;

        let wallet_initial_balance_a = wallet
            .get_asset_balance(&AssetId::new(*exchange.asset_a_id))
            .await
            .unwrap();
        let wallet_initial_balance_b = wallet
            .get_asset_balance(&AssetId::new(*exchange.asset_b_id))
            .await
            .unwrap();
        let initial_pool_info = pool_info(&exchange.contract).await.value;

        let output_amount = swap_with_exact_input(
            &exchange.contract,
            CallParameters::new(
                Some(input_amount),
                Some(AssetId::new(*exchange.asset_a_id)),
                None,
            ),
            None,
            deadline,
        )
        .await
        .value;

        let wallet_final_balance_a = wallet
            .get_asset_balance(&AssetId::new(*exchange.asset_a_id))
            .await
            .unwrap();
        let wallet_final_balance_b = wallet
            .get_asset_balance(&AssetId::new(*exchange.asset_b_id))
            .await
            .unwrap();
        let final_pool_info = pool_info(&exchange.contract).await.value;

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
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_unitialized() {
        // call setup instead of setup_and_initialize
        let (exchange_instance, _wallet, _pool_asset_id, asset_a_id, _asset_b_id, _asset_c_id) =
            setup().await;
        let deadline = 1000;

        swap_with_exact_input(
            &exchange_instance,
            CallParameters::new(Some(1), Some(AssetId::new(*asset_a_id)), None),
            None,
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

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            liquidity,
            deadline,
        )
        .await;

        swap_with_exact_input(
            &exchange.contract,
            // sending invalid asset
            CallParameters::new(Some(1), Some(AssetId::new(*asset_c_id)), None),
            None,
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

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            liquidity,
            deadline,
        )
        .await;

        swap_with_exact_input(
            &exchange.contract,
            CallParameters::new(Some(1), Some(AssetId::new(*exchange.asset_a_id)), None),
            None,
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

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            liquidity,
            deadline,
        )
        .await;

        swap_with_exact_input(
            &exchange.contract,
            // forwarding 0 as msg_amount
            CallParameters::new(Some(0), Some(AssetId::new(*exchange.asset_a_id)), None),
            None,
            deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_minimum_a_constraint_is_too_high() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 400;
        let liquidity = 200;
        let deadline = 1000;
        let input_amount = 10;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            liquidity,
            deadline,
        )
        .await;

        let preview_amount =
            preview_swap_with_exact_input(&exchange.contract, input_amount, exchange.asset_a_id)
                .await
                .value
                .amount;

        swap_with_exact_input(
            &exchange.contract,
            CallParameters::new(
                Some(input_amount),
                Some(AssetId::new(*exchange.asset_a_id)),
                None,
            ),
            // setting min too high
            Some(preview_amount + 1),
            deadline,
        )
        .await;
    }
}
