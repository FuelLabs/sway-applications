use crate::utils::{
    abi_calls::pool_info,
    test_helpers::{deposit_and_add_liquidity, setup, setup_and_initialize},
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn returns_empty_pool_info() {
        let (exchange, _wallet, _amounts, _asset_c_id) = setup_and_initialize().await;
        let pool_info = pool_info(&exchange.instance).await.value;

        assert_eq!(pool_info.asset_a, ContractId::new(*exchange.asset_a));
        assert_eq!(pool_info.asset_a_reserve, 0);
        assert_eq!(pool_info.asset_b, ContractId::new(*exchange.asset_b));
        assert_eq!(pool_info.asset_b_reserve, 0);
        assert_eq!(pool_info.liquidity, 0);
    }

    #[tokio::test]
    async fn returns_non_empty_pool_info() {
        let (exchange, _wallet, amounts, _asset_c_id) = setup_and_initialize().await;

        let initial_pool_info = pool_info(&exchange.instance).await.value;

        deposit_and_add_liquidity(&amounts, &exchange).await;

        let final_pool_info = pool_info(&exchange.instance).await.value;

        assert_eq!(
            initial_pool_info.asset_a,
            ContractId::new(*exchange.asset_a)
        );
        assert_eq!(initial_pool_info.asset_a_reserve, 0);
        assert_eq!(
            initial_pool_info.asset_b,
            ContractId::new(*exchange.asset_b)
        );
        assert_eq!(initial_pool_info.asset_b_reserve, 0);
        assert_eq!(initial_pool_info.liquidity, 0);
        assert_eq!(final_pool_info.asset_a, ContractId::new(*exchange.asset_a));
        assert_eq!(final_pool_info.asset_a_reserve, amounts.amount_a);
        assert_eq!(final_pool_info.asset_b, ContractId::new(*exchange.asset_b));
        assert_eq!(final_pool_info.asset_b_reserve, amounts.amount_b);
        assert_eq!(
            final_pool_info.liquidity * final_pool_info.liquidity,
            amounts.amount_a * amounts.amount_b
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
