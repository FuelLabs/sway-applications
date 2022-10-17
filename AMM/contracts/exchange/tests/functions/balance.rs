use crate::utils::{
    abi_calls::{balance, deposit},
    test_helpers::{setup, setup_and_initialize},
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn returns_zero() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let balance = balance(&exchange.contract, exchange.asset_a_id).await.value;
        assert_eq!(balance, 0);
    }

    #[tokio::test]
    async fn returns_non_zero() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let initial_balance = balance(&exchange.contract, exchange.asset_a_id).await.value;
        assert_eq!(initial_balance, 0);

        let deposit_amount = 10;
        deposit(
            &exchange.contract,
            CallParameters::new(
                Some(deposit_amount),
                Some(AssetId::new(*exchange.asset_a_id)),
                None,
            ),
        )
        .await;

        let balance = balance(&exchange.contract, exchange.asset_a_id).await.value;
        assert_eq!(balance, deposit_amount);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_unitialized() {
        // call setup instead of setup_and_initialize
        let (exchange_instance, _wallet, _pool_asset_id, asset_a_id, _asset_b_id, _asset_c_id) =
            setup().await;
        balance(&exchange_instance, asset_a_id).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_msg_asset_id_is_invalid() {
        let (exchange, _wallet, asset_c_id) = setup_and_initialize().await;
        // send invalid asset id
        balance(&exchange.contract, asset_c_id).await;
    }
}
