use crate::utils::{
    abi_calls::pool_info,
    test_helpers::{deposit_and_add_liquidity, setup, setup_and_initialize},
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn returns_empty_pool_info() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let pool_info = pool_info(&exchange.contract).await.value;
        assert_eq!(pool_info.asset_a, exchange.asset_a_id);
        assert_eq!(pool_info.asset_a_reserve, 0);
        assert_eq!(pool_info.asset_b, exchange.asset_b_id);
        assert_eq!(pool_info.asset_b_reserve, 0);
        assert_eq!(pool_info.liquidity, 0);
    }

    #[tokio::test]
    async fn returns_non_empty_pool_info() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let initial_pool_info = pool_info(&exchange.contract).await.value;
        assert_eq!(initial_pool_info.asset_a, exchange.asset_a_id);
        assert_eq!(initial_pool_info.asset_a_reserve, 0);
        assert_eq!(initial_pool_info.asset_b, exchange.asset_b_id);
        assert_eq!(initial_pool_info.asset_b_reserve, 0);
        assert_eq!(initial_pool_info.liquidity, 0);

        let deposit_amount_a = 100;
        let deposit_amount_b = 400;
        let liquidity = 200;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            liquidity,
            1000,
        )
        .await;

        let pool_info = pool_info(&exchange.contract).await.value;
        assert_eq!(pool_info.asset_a, exchange.asset_a_id);
        assert_eq!(pool_info.asset_a_reserve, deposit_amount_a);
        assert_eq!(pool_info.asset_b, exchange.asset_b_id);
        assert_eq!(pool_info.asset_b_reserve, deposit_amount_b);
        assert_eq!(
            pool_info.liquidity * pool_info.liquidity,
            deposit_amount_a * deposit_amount_b
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
        pool_info(&exchange_instance).await;
    }
}
