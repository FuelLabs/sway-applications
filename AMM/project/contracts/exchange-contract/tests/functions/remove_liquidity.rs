use crate::utils::{setup, setup_and_construct, wallet_balances};
use fuels::prelude::*;
use test_utils::abi::exchange::{pool_info, remove_liquidity};

mod success {
    use super::*;

    #[tokio::test]
    async fn removes_all_liquidity_passing_exact_a_and_b_values() {
        let (exchange, wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let liquidity_to_remove = liquidity_parameters.liquidity;
        let a_to_remove = liquidity_parameters.amounts.0;
        let b_to_remove = liquidity_parameters.amounts.1;
        let expected_liquidity_removed = liquidity_parameters.liquidity;
        let expected_a_removed = liquidity_parameters.amounts.0;
        let expected_b_removed = liquidity_parameters.amounts.1;

        let initial_pool_info = pool_info(&exchange.instance).await.value;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;

        let remove_liquidity_info = remove_liquidity(
            &exchange.instance,
            CallParameters::new(
                Some(liquidity_to_remove),
                Some(AssetId::new(*exchange.id)),
                None,
            ),
            a_to_remove,
            b_to_remove,
            liquidity_parameters.deadline,
        )
        .await
        .value;

        let final_pool_info = pool_info(&exchange.instance).await.value;
        let final_wallet_balances = wallet_balances(&exchange, &wallet).await;

        assert_eq!(remove_liquidity_info.asset_a_amount, expected_a_removed);
        assert_eq!(remove_liquidity_info.asset_b_amount, expected_b_removed);
        assert_eq!(remove_liquidity_info.liquidity, expected_liquidity_removed);
        assert_eq!(
            final_wallet_balances.asset_a,
            initial_wallet_balances.asset_a + expected_a_removed
        );
        assert_eq!(
            final_wallet_balances.asset_b,
            initial_wallet_balances.asset_b + expected_b_removed
        );
        assert_eq!(
            final_wallet_balances.liquidity_pool_asset,
            initial_wallet_balances.liquidity_pool_asset - expected_liquidity_removed
        );
        assert_eq!(
            final_pool_info.asset_a_reserve,
            initial_pool_info.asset_a_reserve - expected_a_removed
        );
        assert_eq!(
            final_pool_info.asset_b_reserve,
            initial_pool_info.asset_b_reserve - expected_b_removed
        );
        assert_eq!(
            final_pool_info.liquidity,
            initial_pool_info.liquidity - expected_liquidity_removed
        );
    }

    #[tokio::test]
    async fn removes_all_liquidity_passing_exact_a_but_not_exact_b_values() {
        let (exchange, wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let liquidity_to_remove = liquidity_parameters.liquidity;
        let a_to_remove = liquidity_parameters.amounts.0;
        let b_to_remove = liquidity_parameters.amounts.1 / 2;
        let expected_liquidity_removed = liquidity_parameters.liquidity;
        let expected_a_removed = liquidity_parameters.amounts.0;
        let expected_b_removed = liquidity_parameters.amounts.1;

        let initial_pool_info = pool_info(&exchange.instance).await.value;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;

        let remove_liquidity_info = remove_liquidity(
            &exchange.instance,
            CallParameters::new(
                Some(liquidity_to_remove),
                Some(AssetId::new(*exchange.id)),
                None,
            ),
            a_to_remove,
            b_to_remove,
            liquidity_parameters.deadline,
        )
        .await
        .value;

        let final_pool_info = pool_info(&exchange.instance).await.value;
        let final_wallet_balances = wallet_balances(&exchange, &wallet).await;

        assert_eq!(remove_liquidity_info.asset_a_amount, expected_a_removed);
        assert_eq!(remove_liquidity_info.asset_b_amount, expected_b_removed);
        assert_eq!(remove_liquidity_info.liquidity, expected_liquidity_removed);
        assert_eq!(
            final_wallet_balances.asset_a,
            initial_wallet_balances.asset_a + expected_a_removed
        );
        assert_eq!(
            final_wallet_balances.asset_b,
            initial_wallet_balances.asset_b + expected_b_removed
        );
        assert_eq!(
            final_wallet_balances.liquidity_pool_asset,
            initial_wallet_balances.liquidity_pool_asset - expected_liquidity_removed
        );
        assert_eq!(
            final_pool_info.asset_a_reserve,
            initial_pool_info.asset_a_reserve - expected_a_removed
        );
        assert_eq!(
            final_pool_info.asset_b_reserve,
            initial_pool_info.asset_b_reserve - expected_b_removed
        );
        assert_eq!(
            final_pool_info.liquidity,
            initial_pool_info.liquidity - expected_liquidity_removed
        );
    }

    #[tokio::test]
    async fn removes_all_liquidity_passing_exact_b_but_not_exact_a_values() {
        let (exchange, wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let liquidity_to_remove = liquidity_parameters.liquidity;
        let a_to_remove = liquidity_parameters.amounts.0 / 2;
        let b_to_remove = liquidity_parameters.amounts.1;
        let expected_liquidity_removed = liquidity_parameters.liquidity;
        let expected_a_removed = liquidity_parameters.amounts.0;
        let expected_b_removed = liquidity_parameters.amounts.1;

        let initial_pool_info = pool_info(&exchange.instance).await.value;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;

        let remove_liquidity_info = remove_liquidity(
            &exchange.instance,
            CallParameters::new(
                Some(liquidity_to_remove),
                Some(AssetId::new(*exchange.id)),
                None,
            ),
            a_to_remove,
            b_to_remove,
            liquidity_parameters.deadline,
        )
        .await
        .value;

        let final_pool_info = pool_info(&exchange.instance).await.value;
        let final_wallet_balances = wallet_balances(&exchange, &wallet).await;

        assert_eq!(remove_liquidity_info.asset_a_amount, expected_a_removed);
        assert_eq!(remove_liquidity_info.asset_b_amount, expected_b_removed);
        assert_eq!(remove_liquidity_info.liquidity, expected_liquidity_removed);
        assert_eq!(
            final_wallet_balances.asset_a,
            initial_wallet_balances.asset_a + expected_a_removed
        );
        assert_eq!(
            final_wallet_balances.asset_b,
            initial_wallet_balances.asset_b + expected_b_removed
        );
        assert_eq!(
            final_wallet_balances.liquidity_pool_asset,
            initial_wallet_balances.liquidity_pool_asset - expected_liquidity_removed
        );
        assert_eq!(
            final_pool_info.asset_a_reserve,
            initial_pool_info.asset_a_reserve - expected_a_removed
        );
        assert_eq!(
            final_pool_info.asset_b_reserve,
            initial_pool_info.asset_b_reserve - expected_b_removed
        );
        assert_eq!(
            final_pool_info.liquidity,
            initial_pool_info.liquidity - expected_liquidity_removed
        );
    }

    #[tokio::test]
    async fn removes_partial_liquidity() {
        let (exchange, wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let liquidity_to_remove = liquidity_parameters.liquidity / 2;
        let a_to_remove = liquidity_parameters.amounts.0 / 2;
        let b_to_remove = liquidity_parameters.amounts.1 / 2;
        let expected_liquidity_removed = liquidity_parameters.liquidity / 2;
        let expected_a_removed = liquidity_parameters.amounts.0 / 2;
        let expected_b_removed = liquidity_parameters.amounts.1 / 2;

        let initial_pool_info = pool_info(&exchange.instance).await.value;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;

        let remove_liquidity_info = remove_liquidity(
            &exchange.instance,
            CallParameters::new(
                Some(liquidity_to_remove),
                Some(AssetId::new(*exchange.id)),
                None,
            ),
            a_to_remove,
            b_to_remove,
            liquidity_parameters.deadline,
        )
        .await
        .value;

        let final_pool_info = pool_info(&exchange.instance).await.value;
        let final_wallet_balances = wallet_balances(&exchange, &wallet).await;

        assert_eq!(remove_liquidity_info.asset_a_amount, expected_a_removed);
        assert_eq!(remove_liquidity_info.asset_b_amount, expected_b_removed);
        assert_eq!(remove_liquidity_info.liquidity, expected_liquidity_removed);
        assert_eq!(
            final_wallet_balances.asset_a,
            initial_wallet_balances.asset_a + expected_a_removed
        );
        assert_eq!(
            final_wallet_balances.asset_b,
            initial_wallet_balances.asset_b + expected_b_removed
        );
        assert_eq!(
            final_wallet_balances.liquidity_pool_asset,
            initial_wallet_balances.liquidity_pool_asset - expected_liquidity_removed
        );
        assert_eq!(
            final_pool_info.asset_a_reserve,
            initial_pool_info.asset_a_reserve - expected_a_removed
        );
        assert_eq!(
            final_pool_info.asset_b_reserve,
            initial_pool_info.asset_b_reserve - expected_b_removed
        );
        assert_eq!(
            final_pool_info.liquidity,
            initial_pool_info.liquidity - expected_liquidity_removed
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "RevertTransactionError(\"NotInitialized\"")]
    async fn when_unitialized() {
        // call setup instead of setup_and_initialize
        let (exchange_instance, _wallet, _pool_asset_id, _asset_a_id, _asset_b_id, _asset_c_id) =
            setup().await;
        let a_to_remove = 1;
        let b_to_remove = 1;
        let deadline = 1000;

        remove_liquidity(
            &exchange_instance,
            CallParameters::new(
                Some(1),
                // Sending `None` instead of `Some(AssetId::new(*pool_asset_id))`
                // because liquidity pool asset does not exist yet.
                // Normally, this also causes Revert(18446744073709486080),
                // but this test condition (not initialized contract) reverts before that.
                None,
                None,
            ),
            a_to_remove,
            b_to_remove,
            deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "RevertTransactionError(\"InvalidAsset\"")]
    async fn when_msg_asset_id_is_not_liquidity_pool_asset_id() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let a_to_remove = 1;
        let b_to_remove = 1;

        remove_liquidity(
            &exchange.instance,
            CallParameters::new(
                Some(liquidity_parameters.liquidity),
                // sending an asset other than pool asset
                Some(exchange.pair.0),
                None,
            ),
            a_to_remove,
            b_to_remove,
            liquidity_parameters.deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "RevertTransactionError(\"AmountCannotBeZero\"")]
    async fn when_minimum_a_amount_is_zero() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let b_to_remove = 1;

        remove_liquidity(
            &exchange.instance,
            CallParameters::new(
                Some(liquidity_parameters.liquidity),
                Some(AssetId::new(*exchange.id)),
                None,
            ),
            // passing 0 as min_asset_a
            0,
            b_to_remove,
            liquidity_parameters.deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "RevertTransactionError(\"AmountCannotBeZero\"")]
    async fn when_minimum_b_amount_is_zero() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let a_to_remove = 1;

        remove_liquidity(
            &exchange.instance,
            CallParameters::new(
                Some(liquidity_parameters.liquidity),
                Some(AssetId::new(*exchange.id)),
                None,
            ),
            a_to_remove,
            // passing 0 as min_asset_b
            0,
            liquidity_parameters.deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "RevertTransactionError(\"DeadlinePassed(0)\"")]
    async fn when_deadline_has_passed() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let a_to_remove = 1;
        let b_to_remove = 1;

        remove_liquidity(
            &exchange.instance,
            CallParameters::new(
                Some(liquidity_parameters.liquidity),
                Some(AssetId::new(*exchange.id)),
                None,
            ),
            a_to_remove,
            b_to_remove,
            // passing 0 as deadline
            0,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "RevertTransactionError(\"AmountCannotBeZero\"")]
    async fn when_msg_amount_is_zero() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let a_to_remove = 1;
        let b_to_remove = 1;

        remove_liquidity(
            &exchange.instance,
            CallParameters::new(
                // sending 0 msg_amount
                Some(0),
                Some(AssetId::new(*exchange.id)),
                None,
            ),
            a_to_remove,
            b_to_remove,
            liquidity_parameters.deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "RevertTransactionError(\"LiquidityCannotBeZero\"")]
    async fn when_liquidity_is_zero() {
        // not adding liquidity to contract before attempting to remove
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, false).await;
        let a_to_remove = 1;
        let b_to_remove = 1;

        remove_liquidity(
            &exchange.instance,
            CallParameters::new(
                Some(1),
                // Sending `None` instead of `Some(AssetId::new(*exchange.id))`
                // because liquidity pool asset does not exist yet.
                // Normally, this also causes Revert(18446744073709486080),
                // but this test condition (zero liquidity) reverts before that.
                None,
                None,
            ),
            a_to_remove,
            b_to_remove,
            liquidity_parameters.deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "RevertTransactionError(\"DesiredAmountTooHigh(110)\"")]
    async fn when_a_reserve_is_insufficient() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let b_to_remove = 1;

        let pool_info = pool_info(&exchange.instance).await.value;
        let asset_a_reserve = pool_info.asset_a_reserve;
        let liquidity = pool_info.liquidity;
        let asset_a_amount_to_remove =
            (liquidity_parameters.liquidity * asset_a_reserve) / liquidity;

        remove_liquidity(
            &exchange.instance,
            CallParameters::new(
                Some(liquidity_parameters.liquidity),
                Some(AssetId::new(*exchange.id)),
                None,
            ),
            // setting min_asset_a to be higher than what can be removed
            asset_a_amount_to_remove + 10,
            b_to_remove,
            liquidity_parameters.deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "RevertTransactionError(\"DesiredAmountTooHigh(410)\"")]
    async fn when_b_reserve_is_insufficient() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let a_to_remove = 1;

        let pool_info = pool_info(&exchange.instance).await.value;
        let asset_b_reserve = pool_info.asset_b_reserve;
        let liquidity = pool_info.liquidity;
        let asset_b_amount_to_remove =
            (liquidity_parameters.liquidity * asset_b_reserve) / liquidity;

        remove_liquidity(
            &exchange.instance,
            CallParameters::new(
                Some(liquidity_parameters.liquidity),
                Some(AssetId::new(*exchange.id)),
                None,
            ),
            a_to_remove,
            // setting min_asset_b to be higher than what can be removed
            asset_b_amount_to_remove + 10,
            liquidity_parameters.deadline,
        )
        .await;
    }
}
