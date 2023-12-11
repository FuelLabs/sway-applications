use test_utils::interface::exchange::pool_info;

mod success {
    use super::*;
    use crate::utils::setup_and_construct;
    use test_utils::setup::common::deposit_and_add_liquidity;

    #[tokio::test]
    async fn returns_empty_pool_info() {
        let (exchange, _wallet, _liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;
        let pool_info = pool_info(&exchange.instance).await;

        assert_eq!(pool_info.reserves.a.id, exchange.pair.0);
        assert_eq!(pool_info.reserves.a.amount, 0);
        assert_eq!(pool_info.reserves.b.id, exchange.pair.1);
        assert_eq!(pool_info.reserves.b.amount, 0);
        assert_eq!(pool_info.liquidity, 0);
    }

    #[tokio::test]
    async fn returns_non_empty_pool_info() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;

        let initial_pool_info = pool_info(&exchange.instance).await;

        deposit_and_add_liquidity(&liquidity_parameters, &exchange, false).await;

        let final_pool_info = pool_info(&exchange.instance).await;

        assert_eq!(initial_pool_info.reserves.a.id, exchange.pair.0);
        assert_eq!(initial_pool_info.reserves.a.amount, 0);
        assert_eq!(initial_pool_info.reserves.b.id, exchange.pair.1);
        assert_eq!(initial_pool_info.reserves.b.amount, 0);
        assert_eq!(initial_pool_info.liquidity, 0);
        assert_eq!(final_pool_info.reserves.a.id, exchange.pair.0);
        assert_eq!(
            final_pool_info.reserves.a.amount,
            liquidity_parameters.amounts.0
        );
        assert_eq!(final_pool_info.reserves.b.id, exchange.pair.1);
        assert_eq!(
            final_pool_info.reserves.b.amount,
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
    use crate::utils::setup;

    #[tokio::test]
    #[should_panic(expected = "AssetPairNotSet")]
    async fn when_uninitialized() {
        // call setup instead of setup_and_construct
        let (exchange_instance, _wallet, _assets, _deadline) = setup().await;

        pool_info(&exchange_instance).await;
    }
}
