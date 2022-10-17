use crate::utils::{
    abi_calls::{pool_info, preview_swap_with_minimum, swap_with_minimum},
    test_helpers::{deposit_and_add_liquidity, setup, setup_and_initialize},
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn swaps_a_for_b() {
        let (exchange, wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 200;
        let swap_amount = 10;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            1000,
            2,
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

        let expected_amount = preview_swap_with_minimum(
            &exchange.contract,
            CallParameters::new(None, Some(AssetId::new(*exchange.asset_a_id)), None),
            swap_amount,
        )
        .await
        .value
        .amount;

        let swapped_amount = swap_with_minimum(
            &exchange.contract,
            CallParameters::new(
                Some(swap_amount),
                Some(AssetId::new(*exchange.asset_a_id)),
                None,
            ),
            1000,
            expected_amount,
        )
        .await
        .value;

        assert_eq!(swapped_amount, expected_amount);

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
            wallet_initial_balance_a - swap_amount
        );
        assert_eq!(
            wallet_final_balance_b,
            wallet_initial_balance_b + expected_amount
        );
        assert_eq!(
            final_pool_info.asset_a_reserve,
            initial_pool_info.asset_a_reserve + swap_amount
        );
        assert_eq!(
            final_pool_info.asset_b_reserve,
            initial_pool_info.asset_b_reserve - expected_amount
        );
    }

    #[tokio::test]
    async fn swaps_b_for_a() {
        let (exchange, wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 200;
        let swap_amount = 10;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            1000,
            2,
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

        let expected_amount = preview_swap_with_minimum(
            &exchange.contract,
            CallParameters::new(None, Some(AssetId::new(*exchange.asset_b_id)), None),
            swap_amount,
        )
        .await
        .value
        .amount;

        let swapped_amount = swap_with_minimum(
            &exchange.contract,
            CallParameters::new(
                Some(swap_amount),
                Some(AssetId::new(*exchange.asset_b_id)),
                None,
            ),
            1000,
            expected_amount,
        )
        .await
        .value;

        assert_eq!(swapped_amount, expected_amount);

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
            wallet_final_balance_b,
            wallet_initial_balance_b - swap_amount
        );
        assert_eq!(
            wallet_final_balance_a,
            wallet_initial_balance_a + expected_amount
        );
        assert_eq!(
            final_pool_info.asset_b_reserve,
            initial_pool_info.asset_b_reserve + expected_amount
        );
        assert_eq!(
            final_pool_info.asset_a_reserve,
            initial_pool_info.asset_a_reserve - expected_amount
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

        swap_with_minimum(
            &exchange_instance,
            CallParameters::new(Some(1), Some(AssetId::new(*asset_a_id)), None),
            1000,
            10,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_msg_asset_id_is_invalid() {
        let (exchange, _wallet, asset_c_id) = setup_and_initialize().await;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            100,
            AssetId::new(*exchange.asset_b_id),
            200,
            1000,
            2,
        )
        .await;

        swap_with_minimum(
            &exchange.contract,
            // sending invalid asset
            CallParameters::new(Some(1), Some(AssetId::new(*asset_c_id)), None),
            1000,
            10,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_swap_amount_is_zero() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            100,
            AssetId::new(*exchange.asset_b_id),
            200,
            1000,
            2,
        )
        .await;

        swap_with_minimum(
            &exchange.contract,
            CallParameters::new(Some(1), Some(AssetId::new(*exchange.asset_a_id)), None),
            1000,
            // passing 0 amount
            0,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_deadline_has_passed() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            100,
            AssetId::new(*exchange.asset_b_id),
            200,
            1000,
            2,
        )
        .await;

        swap_with_minimum(
            &exchange.contract,
            CallParameters::new(Some(1), Some(AssetId::new(*exchange.asset_a_id)), None),
            // passing 0 deadline
            0,
            10,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_msg_amount_is_zero() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            100,
            AssetId::new(*exchange.asset_b_id),
            200,
            1000,
            2,
        )
        .await;

        swap_with_minimum(
            &exchange.contract,
            // forwarding 0 as msg_amount
            CallParameters::new(Some(0), Some(AssetId::new(*exchange.asset_a_id)), None),
            10,
            1000,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_minimum_a_constraint_is_too_high() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let swap_amount = 10;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            100,
            AssetId::new(*exchange.asset_b_id),
            200,
            1000,
            2,
        )
        .await;

        let preview_amount = preview_swap_with_minimum(
            &exchange.contract,
            CallParameters::new(None, Some(AssetId::new(*exchange.asset_a_id)), None),
            swap_amount,
        )
        .await
        .value
        .amount;
        // setting min too high
        let min = preview_amount + 1;

        swap_with_minimum(
            &exchange.contract,
            CallParameters::new(
                Some(swap_amount),
                Some(AssetId::new(*exchange.asset_a_id)),
                None,
            ),
            1000,
            min,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_minimum_b_constraint_is_too_high() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let swap_amount = 10;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            100,
            AssetId::new(*exchange.asset_b_id),
            200,
            1000,
            2,
        )
        .await;

        let preview_amount = preview_swap_with_minimum(
            &exchange.contract,
            CallParameters::new(None, Some(AssetId::new(*exchange.asset_b_id)), None),
            swap_amount,
        )
        .await
        .value
        .amount;
        // setting min too high
        let min = preview_amount + 1;

        swap_with_minimum(
            &exchange.contract,
            CallParameters::new(
                Some(swap_amount),
                Some(AssetId::new(*exchange.asset_b_id)),
                None,
            ),
            1000,
            min,
        )
        .await;
    }
}
