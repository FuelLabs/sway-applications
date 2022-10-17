use crate::utils::{
    abi_calls::{pool_info, remove_liquidity},
    test_helpers::{deposit_and_add_liquidity, setup, setup_and_initialize},
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn removes_all_liquidity_passing_exact_a_and_b_values() {
        let (exchange, wallet, _asset_c_id) = setup_and_initialize().await;

        let deposit_amount_a = 100;
        let deposit_amount_b = 200;

        let added_liquidity = deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            100,
            AssetId::new(*exchange.asset_b_id),
            200,
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
        let wallet_initial_balance_lp = wallet
            .get_asset_balance(&AssetId::new(*exchange.liquidity_pool_id))
            .await
            .unwrap();
        let initial_pool_info = pool_info(&exchange.contract).await.value;

        let liquidity_to_remove = added_liquidity;
        let a_to_remove = deposit_amount_a;
        let b_to_remove = deposit_amount_b;

        let remove_liquidity_info = remove_liquidity(
            &exchange.contract,
            CallParameters::new(
                Some(liquidity_to_remove),
                Some(AssetId::new(*exchange.liquidity_pool_id)),
                Some(100_000_000),
            ),
            TxParameters {
                gas_price: 0,
                gas_limit: 100_000_000,
                maturity: 0,
            },
            1000,
            a_to_remove,
            b_to_remove / 2,
        )
        .await
        .value;

        assert_eq!(remove_liquidity_info.asset_a_amount, a_to_remove);
        assert_eq!(remove_liquidity_info.asset_b_amount, b_to_remove);
        assert_eq!(remove_liquidity_info.liquidity, liquidity_to_remove);

        let wallet_final_balance_a = wallet
            .get_asset_balance(&AssetId::new(*exchange.asset_a_id))
            .await
            .unwrap();
        let wallet_final_balance_b = wallet
            .get_asset_balance(&AssetId::new(*exchange.asset_b_id))
            .await
            .unwrap();
        let wallet_final_balance_lp = wallet
            .get_asset_balance(&AssetId::new(*exchange.liquidity_pool_id))
            .await
            .unwrap();
        let final_pool_info = pool_info(&exchange.contract).await.value;

        assert_eq!(
            wallet_final_balance_a,
            wallet_initial_balance_a + a_to_remove
        );
        assert_eq!(
            wallet_final_balance_b,
            wallet_initial_balance_b + b_to_remove
        );
        assert_eq!(
            wallet_final_balance_lp,
            wallet_initial_balance_lp - liquidity_to_remove
        );
        assert_eq!(
            final_pool_info.asset_a_reserve,
            initial_pool_info.asset_a_reserve - a_to_remove
        );
        assert_eq!(
            final_pool_info.asset_b_reserve,
            initial_pool_info.asset_b_reserve - b_to_remove
        );
        assert_eq!(
            final_pool_info.liquidity,
            initial_pool_info.liquidity - liquidity_to_remove
        );
    }

    #[tokio::test]
    async fn removes_all_liquidity_passing_exact_a_but_not_exact_b_values() {
        let (exchange, wallet, _asset_c_id) = setup_and_initialize().await;

        let deposit_amount_a = 100;
        let deposit_amount_b = 200;

        let added_liquidity = deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            100,
            AssetId::new(*exchange.asset_b_id),
            200,
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
        let wallet_initial_balance_lp = wallet
            .get_asset_balance(&AssetId::new(*exchange.liquidity_pool_id))
            .await
            .unwrap();
        let initial_pool_info = pool_info(&exchange.contract).await.value;

        let liquidity_to_remove = added_liquidity;
        let a_to_remove = deposit_amount_a;
        let b_to_remove = deposit_amount_b;

        let remove_liquidity_info = remove_liquidity(
            &exchange.contract,
            CallParameters::new(
                Some(liquidity_to_remove),
                Some(AssetId::new(*exchange.liquidity_pool_id)),
                Some(100_000_000),
            ),
            TxParameters {
                gas_price: 0,
                gas_limit: 100_000_000,
                maturity: 0,
            },
            1000,
            a_to_remove,
            b_to_remove / 2,
        )
        .await
        .value;

        assert_eq!(remove_liquidity_info.asset_a_amount, a_to_remove);
        assert_eq!(remove_liquidity_info.asset_b_amount, b_to_remove);
        assert_eq!(remove_liquidity_info.liquidity, liquidity_to_remove);

        let wallet_final_balance_a = wallet
            .get_asset_balance(&AssetId::new(*exchange.asset_a_id))
            .await
            .unwrap();
        let wallet_final_balance_b = wallet
            .get_asset_balance(&AssetId::new(*exchange.asset_b_id))
            .await
            .unwrap();
        let wallet_final_balance_lp = wallet
            .get_asset_balance(&AssetId::new(*exchange.liquidity_pool_id))
            .await
            .unwrap();
        let final_pool_info = pool_info(&exchange.contract).await.value;

        assert_eq!(
            wallet_final_balance_a,
            wallet_initial_balance_a + a_to_remove
        );
        assert_eq!(
            wallet_final_balance_b,
            wallet_initial_balance_b + b_to_remove
        );
        assert_eq!(
            wallet_final_balance_lp,
            wallet_initial_balance_lp - liquidity_to_remove
        );
        assert_eq!(
            final_pool_info.asset_a_reserve,
            initial_pool_info.asset_a_reserve - a_to_remove
        );
        assert_eq!(
            final_pool_info.asset_b_reserve,
            initial_pool_info.asset_b_reserve - b_to_remove
        );
        assert_eq!(
            final_pool_info.liquidity,
            initial_pool_info.liquidity - liquidity_to_remove
        );
    }

    #[tokio::test]
    async fn removes_all_liquidity_passing_exact_b_but_not_exact_a_values() {
        let (exchange, wallet, _asset_c_id) = setup_and_initialize().await;

        let deposit_amount_a = 100;
        let deposit_amount_b = 200;

        let added_liquidity = deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            100,
            AssetId::new(*exchange.asset_b_id),
            200,
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
        let wallet_initial_balance_lp = wallet
            .get_asset_balance(&AssetId::new(*exchange.liquidity_pool_id))
            .await
            .unwrap();
        let initial_pool_info = pool_info(&exchange.contract).await.value;

        let liquidity_to_remove = added_liquidity;
        let a_to_remove = deposit_amount_a;
        let b_to_remove = deposit_amount_b;

        let remove_liquidity_info = remove_liquidity(
            &exchange.contract,
            CallParameters::new(
                Some(liquidity_to_remove),
                Some(AssetId::new(*exchange.liquidity_pool_id)),
                Some(100_000_000),
            ),
            TxParameters {
                gas_price: 0,
                gas_limit: 100_000_000,
                maturity: 0,
            },
            1000,
            a_to_remove / 2,
            b_to_remove,
        )
        .await
        .value;

        assert_eq!(remove_liquidity_info.asset_a_amount, a_to_remove);
        assert_eq!(remove_liquidity_info.asset_b_amount, b_to_remove);
        assert_eq!(remove_liquidity_info.liquidity, liquidity_to_remove);

        let wallet_final_balance_a = wallet
            .get_asset_balance(&AssetId::new(*exchange.asset_a_id))
            .await
            .unwrap();
        let wallet_final_balance_b = wallet
            .get_asset_balance(&AssetId::new(*exchange.asset_b_id))
            .await
            .unwrap();
        let wallet_final_balance_lp = wallet
            .get_asset_balance(&AssetId::new(*exchange.liquidity_pool_id))
            .await
            .unwrap();
        let final_pool_info = pool_info(&exchange.contract).await.value;

        assert_eq!(
            wallet_final_balance_a,
            wallet_initial_balance_a + a_to_remove
        );
        assert_eq!(
            wallet_final_balance_b,
            wallet_initial_balance_b + b_to_remove
        );
        assert_eq!(
            wallet_final_balance_lp,
            wallet_initial_balance_lp - liquidity_to_remove
        );
        assert_eq!(
            final_pool_info.asset_a_reserve,
            initial_pool_info.asset_a_reserve - a_to_remove
        );
        assert_eq!(
            final_pool_info.asset_b_reserve,
            initial_pool_info.asset_b_reserve - b_to_remove
        );
        assert_eq!(
            final_pool_info.liquidity,
            initial_pool_info.liquidity - liquidity_to_remove
        );
    }

    #[tokio::test]
    async fn removes_partial_liquidity() {
        let (exchange, wallet, _asset_c_id) = setup_and_initialize().await;

        let deposit_amount_a = 100;
        let deposit_amount_b = 200;

        let added_liquidity = deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            100,
            AssetId::new(*exchange.asset_b_id),
            200,
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
        let wallet_initial_balance_lp = wallet
            .get_asset_balance(&AssetId::new(*exchange.liquidity_pool_id))
            .await
            .unwrap();
        let initial_pool_info = pool_info(&exchange.contract).await.value;

        let liquidity_to_remove = added_liquidity / 2;
        let a_to_remove = deposit_amount_a / 2;
        let b_to_remove = deposit_amount_b / 2;

        let remove_liquidity_info = remove_liquidity(
            &exchange.contract,
            CallParameters::new(
                Some(liquidity_to_remove),
                Some(AssetId::new(*exchange.liquidity_pool_id)),
                Some(100_000_000),
            ),
            TxParameters {
                gas_price: 0,
                gas_limit: 100_000_000,
                maturity: 0,
            },
            1000,
            a_to_remove,
            b_to_remove,
        )
        .await
        .value;

        assert_eq!(remove_liquidity_info.asset_a_amount, a_to_remove);
        assert_eq!(remove_liquidity_info.asset_b_amount, b_to_remove);
        assert_eq!(remove_liquidity_info.liquidity, liquidity_to_remove);

        let wallet_final_balance_a = wallet
            .get_asset_balance(&AssetId::new(*exchange.asset_a_id))
            .await
            .unwrap();
        let wallet_final_balance_b = wallet
            .get_asset_balance(&AssetId::new(*exchange.asset_b_id))
            .await
            .unwrap();
        let wallet_final_balance_lp = wallet
            .get_asset_balance(&AssetId::new(*exchange.liquidity_pool_id))
            .await
            .unwrap();
        let final_pool_info = pool_info(&exchange.contract).await.value;

        assert_eq!(
            wallet_final_balance_a,
            wallet_initial_balance_a + a_to_remove
        );
        assert_eq!(
            wallet_final_balance_b,
            wallet_initial_balance_b + b_to_remove
        );
        assert_eq!(
            wallet_final_balance_lp,
            wallet_initial_balance_lp - liquidity_to_remove
        );
        assert_eq!(
            final_pool_info.asset_a_reserve,
            initial_pool_info.asset_a_reserve - a_to_remove
        );
        assert_eq!(
            final_pool_info.asset_b_reserve,
            initial_pool_info.asset_b_reserve - b_to_remove
        );
        assert_eq!(
            final_pool_info.liquidity,
            initial_pool_info.liquidity - liquidity_to_remove
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_unitialized() {
        // call setup instead of setup_and_initialize
        let (exchange_instance, _wallet, _pool_asset_id, _asset_a_id, _asset_b_id, _asset_c_id) =
            setup().await;

        remove_liquidity(
            &exchange_instance,
            CallParameters::new(
                Some(1),
                // Sending `None` instead of `Some(AssetId::new(*pool_asset_id))`
                // because liquidity pool asset does not exist yet.
                // Normally, this also causes Revert(42),
                // but this test condition (not initialized contract) reverts before that.
                None,
                Some(100_000_000),
            ),
            TxParameters {
                gas_price: 0,
                gas_limit: 100_000_000,
                maturity: 0,
            },
            1000,
            1,
            1,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_msg_asset_id_is_not_liquidity_pool_asset_id() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;

        let added_liquidity = deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            100,
            AssetId::new(*exchange.asset_b_id),
            200,
            1000,
            2,
        )
        .await;

        remove_liquidity(
            &exchange.contract,
            CallParameters::new(
                Some(added_liquidity),
                // sending an asset other than pool asset
                Some(AssetId::new(*exchange.asset_a_id)),
                Some(100_000_000),
            ),
            TxParameters {
                gas_price: 0,
                gas_limit: 100_000_000,
                maturity: 0,
            },
            1000,
            1,
            1,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_minimum_a_amount_is_zero() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;

        let added_liquidity = deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            100,
            AssetId::new(*exchange.asset_b_id),
            200,
            1000,
            2,
        )
        .await;

        remove_liquidity(
            &exchange.contract,
            CallParameters::new(
                Some(added_liquidity),
                Some(AssetId::new(*exchange.liquidity_pool_id)),
                Some(100_000_000),
            ),
            TxParameters {
                gas_price: 0,
                gas_limit: 100_000_000,
                maturity: 0,
            },
            1000,
            // passing 0 as min_asset_a
            0,
            1,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_minimum_b_amount_is_zero() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;

        let added_liquidity = deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            100,
            AssetId::new(*exchange.asset_b_id),
            200,
            1000,
            2,
        )
        .await;

        remove_liquidity(
            &exchange.contract,
            CallParameters::new(
                Some(added_liquidity),
                Some(AssetId::new(*exchange.liquidity_pool_id)),
                Some(100_000_000),
            ),
            TxParameters {
                gas_price: 0,
                gas_limit: 100_000_000,
                maturity: 0,
            },
            1000,
            1,
            // passing 0 as min_asset_b
            0,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_deadline_has_passed() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;

        let added_liquidity = deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            100,
            AssetId::new(*exchange.asset_b_id),
            200,
            1000,
            2,
        )
        .await;

        remove_liquidity(
            &exchange.contract,
            CallParameters::new(
                Some(added_liquidity),
                Some(AssetId::new(*exchange.liquidity_pool_id)),
                Some(100_000_000),
            ),
            TxParameters {
                gas_price: 0,
                gas_limit: 100_000_000,
                maturity: 0,
            },
            // passing 0 as deadline
            0,
            1,
            1,
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

        remove_liquidity(
            &exchange.contract,
            CallParameters::new(
                // sending 0 msg_amount
                Some(0),
                Some(AssetId::new(*exchange.liquidity_pool_id)),
                Some(100_000_000),
            ),
            TxParameters {
                gas_price: 0,
                gas_limit: 100_000_000,
                maturity: 0,
            },
            1000,
            1,
            1,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_liquidity_is_zero() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;

        // not adding liquidity to contract before attempting to remove

        let pool_info = pool_info(&exchange.contract).await.value;
        assert_eq!(pool_info.liquidity, 0);

        remove_liquidity(
            &exchange.contract,
            CallParameters::new(
                Some(1),
                // Sending `None` instead of `Some(AssetId::new(*exchange.liquidity_pool_id))`
                // because liquidity pool asset does not exist yet.
                // Normally, this also causes Revert(42),
                // but this test condition (zero liquidity) reverts before that.
                None,
                Some(100_000_000),
            ),
            TxParameters {
                gas_price: 0,
                gas_limit: 100_000_000,
                maturity: 0,
            },
            1000,
            1,
            1,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_a_reserve_is_insufficient() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;

        let added_liquidity = deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            100,
            AssetId::new(*exchange.asset_b_id),
            200,
            1000,
            2,
        )
        .await;

        let pool_info = pool_info(&exchange.contract).await.value;
        let asset_a_reserve = pool_info.asset_a_reserve;
        let liquidity = pool_info.liquidity;
        let asset_a_amount_to_remove = (added_liquidity * asset_a_reserve) / liquidity;

        remove_liquidity(
            &exchange.contract,
            CallParameters::new(
                Some(added_liquidity),
                Some(AssetId::new(*exchange.liquidity_pool_id)),
                Some(100_000_000),
            ),
            TxParameters {
                gas_price: 0,
                gas_limit: 100_000_000,
                maturity: 0,
            },
            1000,
            // setting min_asset_a to be higher than what can be removed
            asset_a_amount_to_remove + 10,
            1,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_b_reserve_is_insufficient() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;

        let added_liquidity = deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            100,
            AssetId::new(*exchange.asset_b_id),
            200,
            1000,
            2,
        )
        .await;

        let pool_info = pool_info(&exchange.contract).await.value;
        let asset_b_reserve = pool_info.asset_b_reserve;
        let liquidity = pool_info.liquidity;
        let asset_b_amount_to_remove = (added_liquidity * asset_b_reserve) / liquidity;

        remove_liquidity(
            &exchange.contract,
            CallParameters::new(
                Some(added_liquidity),
                Some(AssetId::new(*exchange.liquidity_pool_id)),
                Some(100_000_000),
            ),
            TxParameters {
                gas_price: 0,
                gas_limit: 100_000_000,
                maturity: 0,
            },
            1000,
            1,
            // setting min_asset_b to be higher than what can be removed
            asset_b_amount_to_remove + 10,
        )
        .await;
    }
}
