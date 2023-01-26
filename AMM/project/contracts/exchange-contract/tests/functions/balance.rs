use crate::utils::{
    abi_calls::{balance, deposit},
    test_helpers::{setup, setup_and_initialize},
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn returns_zero() {
        let (exchange, _wallet, _amounts, _asset_c_id) = setup_and_initialize().await;

        let balance = balance(&exchange.instance, exchange.asset_a).await.value;

        assert_eq!(balance, 0);
    }

    #[tokio::test]
    async fn returns_non_zero() {
        let (exchange, _wallet, _amounts, _asset_c_id) = setup_and_initialize().await;
        let initial_balance = balance(&exchange.instance, exchange.asset_a).await.value;
        let deposit_amount = 10;

        deposit(
            &exchange.instance,
            CallParameters::new(Some(deposit_amount), Some(exchange.asset_a), None),
        )
        .await;
        let balance = balance(&exchange.instance, exchange.asset_a).await.value;

        assert_eq!(initial_balance, 0);
        assert_eq!(balance, deposit_amount);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotInitialized")]
    async fn when_unitialized() {
        // call setup instead of setup_and_initialize
        let (exchange_instance, _wallet, _pool_asset_id, asset_a_id, _asset_b_id, _asset_c_id) =
            setup().await;

        balance(&exchange_instance, asset_a_id).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidAsset")]
    async fn when_msg_asset_id_is_invalid() {
        let (exchange, _wallet, _amounts, asset_c_id) = setup_and_initialize().await;

        // send invalid asset id
        balance(&exchange.instance, asset_c_id).await;
    }
}
