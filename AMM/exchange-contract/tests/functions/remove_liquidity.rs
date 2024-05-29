use crate::utils::setup_and_construct;
use test_utils::interface::exchange::{pool_info, remove_liquidity};

mod success {
    use super::*;
    use crate::utils::wallet_balances;
    use fuels::{
        tx::ContractIdExt,
        types::{AssetId, Bytes32},
    };
    use test_utils::interface::{Asset, AssetPair, RemoveLiquidityEvent};

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

        let initial_pool_info = pool_info(&exchange.instance).await;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;

        let response = remove_liquidity(
            &exchange.instance,
            AssetId::from(*exchange.id.default_asset()),
            liquidity_to_remove,
            a_to_remove,
            b_to_remove,
            liquidity_parameters.deadline,
            true,
        )
        .await;
        let log = response
            .decode_logs_with_type::<RemoveLiquidityEvent>()
            .unwrap();
        let event = log.first().unwrap();

        let remove_liquidity_info = response.value;

        let final_pool_info = pool_info(&exchange.instance).await;
        let final_wallet_balances = wallet_balances(&exchange, &wallet).await;

        assert_eq!(
            *event,
            RemoveLiquidityEvent {
                removed_reserve: AssetPair {
                    a: Asset {
                        id: exchange.pair.0,
                        amount: expected_a_removed,
                    },
                    b: Asset {
                        id: exchange.pair.1,
                        amount: expected_b_removed,
                    }
                },
                burned_liquidity: Asset {
                    id: exchange.id.asset_id(&Bytes32::zeroed()),
                    amount: expected_liquidity_removed,
                }
            }
        );
        assert_eq!(
            remove_liquidity_info.removed_amounts.a.amount,
            expected_a_removed
        );
        assert_eq!(
            remove_liquidity_info.removed_amounts.b.amount,
            expected_b_removed
        );
        assert_eq!(
            remove_liquidity_info.burned_liquidity.amount,
            expected_liquidity_removed
        );
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
            final_pool_info.reserves.a.amount,
            initial_pool_info.reserves.a.amount - expected_a_removed
        );
        assert_eq!(
            final_pool_info.reserves.b.amount,
            initial_pool_info.reserves.b.amount - expected_b_removed
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

        let initial_pool_info = pool_info(&exchange.instance).await;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;

        let response = remove_liquidity(
            &exchange.instance,
            AssetId::from(*exchange.id.default_asset()),
            liquidity_to_remove,
            a_to_remove,
            b_to_remove,
            liquidity_parameters.deadline,
            true,
        )
        .await;
        let log = response
            .decode_logs_with_type::<RemoveLiquidityEvent>()
            .unwrap();
        let event = log.first().unwrap();

        let remove_liquidity_info = response.value;

        let final_pool_info = pool_info(&exchange.instance).await;
        let final_wallet_balances = wallet_balances(&exchange, &wallet).await;

        assert_eq!(
            *event,
            RemoveLiquidityEvent {
                removed_reserve: AssetPair {
                    a: Asset {
                        id: exchange.pair.0,
                        amount: expected_a_removed,
                    },
                    b: Asset {
                        id: exchange.pair.1,
                        amount: expected_b_removed,
                    }
                },
                burned_liquidity: Asset {
                    id: exchange.id.asset_id(&Bytes32::zeroed()),
                    amount: expected_liquidity_removed,
                }
            }
        );
        assert_eq!(
            remove_liquidity_info.removed_amounts.a.amount,
            expected_a_removed
        );
        assert_eq!(
            remove_liquidity_info.removed_amounts.b.amount,
            expected_b_removed
        );
        assert_eq!(
            remove_liquidity_info.burned_liquidity.amount,
            expected_liquidity_removed
        );
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
            final_pool_info.reserves.a.amount,
            initial_pool_info.reserves.a.amount - expected_a_removed
        );
        assert_eq!(
            final_pool_info.reserves.b.amount,
            initial_pool_info.reserves.b.amount - expected_b_removed
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

        let initial_pool_info = pool_info(&exchange.instance).await;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;

        let response = remove_liquidity(
            &exchange.instance,
            AssetId::from(*exchange.id.default_asset()),
            liquidity_to_remove,
            a_to_remove,
            b_to_remove,
            liquidity_parameters.deadline,
            true,
        )
        .await;
        let log = response
            .decode_logs_with_type::<RemoveLiquidityEvent>()
            .unwrap();
        let event = log.first().unwrap();

        let remove_liquidity_info = response.value;

        let final_pool_info = pool_info(&exchange.instance).await;
        let final_wallet_balances = wallet_balances(&exchange, &wallet).await;

        assert_eq!(
            *event,
            RemoveLiquidityEvent {
                removed_reserve: AssetPair {
                    a: Asset {
                        id: exchange.pair.0,
                        amount: expected_a_removed,
                    },
                    b: Asset {
                        id: exchange.pair.1,
                        amount: expected_b_removed,
                    }
                },
                burned_liquidity: Asset {
                    id: exchange.id.asset_id(&Bytes32::zeroed()),
                    amount: expected_liquidity_removed,
                }
            }
        );
        assert_eq!(
            remove_liquidity_info.removed_amounts.a.amount,
            expected_a_removed
        );
        assert_eq!(
            remove_liquidity_info.removed_amounts.b.amount,
            expected_b_removed
        );
        assert_eq!(
            remove_liquidity_info.burned_liquidity.amount,
            expected_liquidity_removed
        );
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
            final_pool_info.reserves.a.amount,
            initial_pool_info.reserves.a.amount - expected_a_removed
        );
        assert_eq!(
            final_pool_info.reserves.b.amount,
            initial_pool_info.reserves.b.amount - expected_b_removed
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

        let initial_pool_info = pool_info(&exchange.instance).await;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;

        let response = remove_liquidity(
            &exchange.instance,
            AssetId::from(*exchange.id.default_asset()),
            liquidity_to_remove,
            a_to_remove,
            b_to_remove,
            liquidity_parameters.deadline,
            true,
        )
        .await;
        let log = response
            .decode_logs_with_type::<RemoveLiquidityEvent>()
            .unwrap();
        let event = log.first().unwrap();

        let remove_liquidity_info = response.value;

        let final_pool_info = pool_info(&exchange.instance).await;
        let final_wallet_balances = wallet_balances(&exchange, &wallet).await;

        assert_eq!(
            *event,
            RemoveLiquidityEvent {
                removed_reserve: AssetPair {
                    a: Asset {
                        id: exchange.pair.0,
                        amount: expected_a_removed,
                    },
                    b: Asset {
                        id: exchange.pair.1,
                        amount: expected_b_removed,
                    }
                },
                burned_liquidity: Asset {
                    id: exchange.id.asset_id(&Bytes32::zeroed()),
                    amount: expected_liquidity_removed,
                }
            }
        );
        assert_eq!(
            remove_liquidity_info.removed_amounts.a.amount,
            expected_a_removed
        );
        assert_eq!(
            remove_liquidity_info.removed_amounts.b.amount,
            expected_b_removed
        );
        assert_eq!(
            remove_liquidity_info.burned_liquidity.amount,
            expected_liquidity_removed
        );
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
            final_pool_info.reserves.a.amount,
            initial_pool_info.reserves.a.amount - expected_a_removed
        );
        assert_eq!(
            final_pool_info.reserves.b.amount,
            initial_pool_info.reserves.b.amount - expected_b_removed
        );
        assert_eq!(
            final_pool_info.liquidity,
            initial_pool_info.liquidity - expected_liquidity_removed
        );
    }
}

mod revert {
    use super::*;
    use crate::utils::setup;
    use fuels::{tx::ContractIdExt, types::AssetId};

    #[tokio::test]
    #[should_panic(expected = "AssetPairNotSet")]
    async fn when_uninitialized() {
        // call setup instead of setup_and_initialize
        let (exchange_instance, _wallet, assets, deadline) = setup().await;
        let a_to_remove = 1;
        let b_to_remove = 1;

        // use fuels::tx::ContractIdExt;
        //             AssetId::from(*exchange_id.default_asset()),

        remove_liquidity(
            &exchange_instance,
            assets.asset_3, // passing another asset since liquidity pool asset does not exist yet
            1,
            a_to_remove,
            b_to_remove,
            deadline,
            false,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "NoLiquidityToRemove")]
    async fn when_liquidity_is_zero() {
        // not adding liquidity to contract before attempting to remove
        let (exchange, _wallet, liquidity_parameters, asset_c_id) =
            setup_and_construct(true, false).await;
        let a_to_remove = 1;
        let b_to_remove = 1;

        remove_liquidity(
            &exchange.instance,
            asset_c_id, // passing another asset since liquidity does not exist yet
            1,
            a_to_remove,
            b_to_remove,
            liquidity_parameters.deadline,
            false,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidAsset")]
    async fn when_msg_asset_id_is_not_liquidity_pool_asset_id() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let a_to_remove = 1;
        let b_to_remove = 1;

        remove_liquidity(
            &exchange.instance,
            exchange.pair.0, // forwarding an asset other than pool asset
            liquidity_parameters.liquidity,
            a_to_remove,
            b_to_remove,
            liquidity_parameters.deadline,
            false,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "ExpectedNonZeroParameter")]
    async fn when_minimum_a_amount_is_zero() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let b_to_remove = 1;

        remove_liquidity(
            &exchange.instance,
            AssetId::from(*exchange.id.default_asset()),
            liquidity_parameters.liquidity,
            0, // passing 0 as min_asset_a
            b_to_remove,
            liquidity_parameters.deadline,
            false,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "ExpectedNonZeroParameter")]
    async fn when_minimum_b_amount_is_zero() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let a_to_remove = 1;

        remove_liquidity(
            &exchange.instance,
            AssetId::from(*exchange.id.default_asset()),
            liquidity_parameters.liquidity,
            a_to_remove,
            0, // passing 0 as min_asset_b
            liquidity_parameters.deadline,
            false,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "DeadlinePassed")]
    async fn when_deadline_passed() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let a_to_remove = 1;
        let b_to_remove = 1;

        remove_liquidity(
            &exchange.instance,
            AssetId::from(*exchange.id.default_asset()),
            liquidity_parameters.liquidity,
            a_to_remove,
            b_to_remove,
            0, // passing 0 as deadline
            false,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "ExpectedNonZeroAmount")]
    async fn when_msg_amount_is_zero() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let a_to_remove = 1;
        let b_to_remove = 1;

        remove_liquidity(
            &exchange.instance,
            AssetId::from(*exchange.id.default_asset()),
            0, // forwarding 0 as msg_amount
            a_to_remove,
            b_to_remove,
            liquidity_parameters.deadline,
            false,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "DesiredAmountTooHigh")]
    async fn when_a_reserve_is_insufficient() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let b_to_remove = 1;

        let pool_info = pool_info(&exchange.instance).await;
        let asset_a_reserve = pool_info.reserves.a.amount;
        let liquidity = pool_info.liquidity;
        let asset_a_amount_to_remove =
            (liquidity_parameters.liquidity * asset_a_reserve) / liquidity;

        remove_liquidity(
            &exchange.instance,
            AssetId::from(*exchange.id.default_asset()),
            liquidity_parameters.liquidity,
            asset_a_amount_to_remove + 10, // setting min_asset_a to be higher than what can be removed
            b_to_remove,
            liquidity_parameters.deadline,
            true,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "DesiredAmountTooHigh")]
    async fn when_b_reserve_is_insufficient() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let a_to_remove = 1;

        let pool_info = pool_info(&exchange.instance).await;
        let asset_b_reserve = pool_info.reserves.b.amount;
        let liquidity = pool_info.liquidity;
        let asset_b_amount_to_remove =
            (liquidity_parameters.liquidity * asset_b_reserve) / liquidity;

        remove_liquidity(
            &exchange.instance,
            AssetId::from(*exchange.id.default_asset()),
            liquidity_parameters.liquidity,
            a_to_remove,
            asset_b_amount_to_remove + 10, // setting min_asset_b to be higher than what can be removed
            liquidity_parameters.deadline,
            true,
        )
        .await;
    }
}
