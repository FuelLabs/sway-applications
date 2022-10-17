use crate::utils::{
    abi_calls::{pool_info, preview_add_liquidity},
    test_helpers::{deposit_and_add_liquidity, setup, setup_and_initialize},
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn previews_adding_a_when_liquidity_is_zero() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;

        let preview_amount = 100;

        let preview = preview_add_liquidity(
            &exchange.contract,
            CallParameters::new(None, Some(AssetId::new(*exchange.asset_a_id)), None),
            TxParameters::default(),
            preview_amount,
            AssetId::new(*exchange.asset_a_id),
        )
        .await
        .value;

        assert_eq!(preview.other_asset_amount_to_add, 0);
        assert_eq!(preview.liquidity_asset_amount_to_receive, preview_amount);
    }

    #[tokio::test]
    async fn previews_adding_a_when_liquidity_is_not_zero() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;

        let deposit_amount_a = 100;
        let deposit_amount_b = 200;
        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            1000,
            2,
        )
        .await;

        let pool_info = pool_info(&exchange.contract).await.value;
        assert_ne!(pool_info.liquidity, 0);

        let preview_amount = 100;

        let preview = preview_add_liquidity(
            &exchange.contract,
            CallParameters::new(None, Some(AssetId::new(*exchange.asset_a_id)), None),
            TxParameters {
                gas_price: 0,
                gas_limit: 100_000_000,
                maturity: 0,
            },
            preview_amount,
            AssetId::new(*exchange.asset_a_id),
        )
        .await
        .value;

        assert_eq!(
            preview.other_asset_amount_to_add,
            (preview_amount * deposit_amount_b) / deposit_amount_a
        );
        assert_eq!(preview.liquidity_asset_amount_to_receive, preview_amount);
    }

    #[tokio::test]
    async fn previews_adding_b_when_liquidity_is_not_zero() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;

        let deposit_amount_a = 100;
        let deposit_amount_b = 200;
        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            1000,
            2,
        )
        .await;

        let pool_info = pool_info(&exchange.contract).await.value;
        assert_ne!(pool_info.liquidity, 0);

        let preview_amount = 100;

        let preview = preview_add_liquidity(
            &exchange.contract,
            CallParameters::new(None, Some(AssetId::new(*exchange.asset_b_id)), None),
            TxParameters {
                gas_price: 0,
                gas_limit: 100_000_000,
                maturity: 0,
            },
            preview_amount,
            AssetId::new(*exchange.asset_b_id),
        )
        .await
        .value;

        assert_eq!(
            preview.other_asset_amount_to_add,
            (preview_amount * deposit_amount_a) / deposit_amount_b
        );
        assert_eq!(
            preview.liquidity_asset_amount_to_receive,
            (preview_amount * deposit_amount_a) / deposit_amount_b
        );
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

        preview_add_liquidity(
            &exchange_instance,
            CallParameters::new(None, Some(AssetId::new(*asset_a_id)), None),
            TxParameters::default(),
            100,
            AssetId::new(*asset_a_id),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_msg_asset_id_is_invalid() {
        let (exchange, _wallet, asset_c_id) = setup_and_initialize().await;

        preview_add_liquidity(
            &exchange.contract,
            CallParameters::new(None, Some(AssetId::new(*asset_c_id)), None),
            TxParameters::default(),
            100,
            AssetId::new(*asset_c_id),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_adding_b_when_liquidity_is_zero() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;

        let preview_amount = 100;

        preview_add_liquidity(
            &exchange.contract,
            CallParameters::new(None, Some(AssetId::new(*exchange.asset_b_id)), None),
            TxParameters::default(),
            preview_amount,
            AssetId::new(*exchange.asset_b_id),
        )
        .await;
    }
}
