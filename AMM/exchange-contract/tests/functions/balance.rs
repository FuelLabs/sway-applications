use crate::utils::setup_and_construct;
use test_utils::interface::exchange::balance;

mod success {
    use super::*;
    use test_utils::interface::exchange::deposit;

    #[tokio::test]
    async fn returns_zero() {
        let (exchange, _wallet, _liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;

        let balance = balance(&exchange.instance, exchange.pair.0).await;

        assert_eq!(balance, 0);
    }

    #[tokio::test]
    async fn returns_non_zero() {
        let (exchange, _wallet, _liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;
        let initial_balance = balance(&exchange.instance, exchange.pair.0).await;

        let deposit_amount = 10;

        deposit(&exchange.instance, deposit_amount, exchange.pair.0).await;

        let balance = balance(&exchange.instance, exchange.pair.0).await;

        assert_eq!(initial_balance, 0);
        assert_eq!(balance, deposit_amount);
    }
}

mod revert {
    use super::*;
    use crate::utils::setup;

    #[tokio::test]
    #[should_panic(expected = "AssetPairNotSet")]
    async fn when_uninitialized() {
        // call setup instead of setup_and_construct
        let (exchange_instance, _wallet, assets, _deadline) = setup().await;

        balance(&exchange_instance, assets.asset_1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidAsset")]
    async fn when_msg_asset_id_is_invalid() {
        let (exchange, _wallet, _liquidity_parameters, asset_c_id) =
            setup_and_construct(false, false).await;

        // send invalid asset id
        balance(&exchange.instance, asset_c_id).await;
    }
}
