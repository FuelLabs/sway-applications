use crate::utils::{
    abi_calls::pool_info,
    test_helpers::{setup, setup_and_initialize},
};

mod success {
    use super::*;

    #[tokio::test]
    async fn can_get_pool_info() {
        let (
            exchange_instance,
            _wallet,
            _pool_asset_id,
            _base_asset_id,
            _other_asset_id,
            _invalid_asset_id,
        ) = setup_and_initialize().await;

        let pool_info = pool_info(&exchange_instance).await;

        assert_eq!(pool_info.base_asset_reserve, 0);
        assert_eq!(pool_info.other_asset_reserve, 0);
        assert_eq!(pool_info.total_liquidity, 0);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "InvalidData")]
    async fn when_not_initialized() {
        // call setup instead of setup_and_initialize
        let (
            exchange_instance,
            _wallet,
            _pool_asset_id,
            _base_asset_id,
            _other_asset_id,
            _invalid_asset_id,
        ) = setup().await;

        pool_info(&exchange_instance).await;
    }
}
