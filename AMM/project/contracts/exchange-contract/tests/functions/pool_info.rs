use crate::utils::{setup, setup_and_construct};
use fuels::prelude::*;
use test_utils::{abi::exchange::pool_info, setup::common::deposit_and_add_liquidity};

mod success {
    use super::*;

    #[tokio::test]
    async fn returns_empty_pool_info() {
        let (exchange, _wallet, _liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;
        let pool_info = pool_info(&exchange.instance).await.value;

        assert_eq!(pool_info.asset_a, ContractId::new(*exchange.pair.0));
        assert_eq!(pool_info.asset_a_reserve, 0);
        assert_eq!(pool_info.asset_b, ContractId::new(*exchange.pair.1));
        assert_eq!(pool_info.asset_b_reserve, 0);
        assert_eq!(pool_info.liquidity, 0);
    }

    #[tokio::test]
    async fn returns_non_empty_pool_info() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;

        let initial_pool_info = pool_info(&exchange.instance).await.value;

        deposit_and_add_liquidity(&liquidity_parameters, &exchange).await;

        let final_pool_info = pool_info(&exchange.instance).await.value;

        assert_eq!(initial_pool_info.asset_a, ContractId::new(*exchange.pair.0));
        assert_eq!(initial_pool_info.asset_a_reserve, 0);
        assert_eq!(initial_pool_info.asset_b, ContractId::new(*exchange.pair.1));
        assert_eq!(initial_pool_info.asset_b_reserve, 0);
        assert_eq!(initial_pool_info.liquidity, 0);
        assert_eq!(final_pool_info.asset_a, ContractId::new(*exchange.pair.0));
        assert_eq!(
            final_pool_info.asset_a_reserve,
            liquidity_parameters.amounts.0
        );
        assert_eq!(final_pool_info.asset_b, ContractId::new(*exchange.pair.1));
        assert_eq!(
            final_pool_info.asset_b_reserve,
            liquidity_parameters.amounts.1
        );
        assert_eq!(
            final_pool_info.liquidity * final_pool_info.liquidity,
            liquidity_parameters.amounts.0 * liquidity_parameters.amounts.1
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "AssetPairNotSet")]
    async fn when_uninitialized() {
        // call setup instead of setup_and_construct
        let (exchange_instance, _wallet, _pool_asset_id, _asset_a_id, _asset_b_id, _asset_c_id) =
            setup().await;

        pool_info(&exchange_instance).await;
    }
}
