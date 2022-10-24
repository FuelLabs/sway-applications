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

        let amount_a = 100;
        let expected_b_to_add = amount_a;
        let expected_liquidity_asset_amount_to_receive_squared = amount_a * expected_b_to_add;

        let preview = preview_add_liquidity(
            &exchange.contract,
            CallParameters::default(),
            TxParameters::default(),
            amount_a,
            AssetId::new(*exchange.asset_a_id),
        )
        .await
        .value;

        assert_eq!(preview.other_asset_amount_to_add, expected_b_to_add);
        assert_eq!(
            preview.liquidity_asset_amount_to_receive * preview.liquidity_asset_amount_to_receive,
            expected_liquidity_asset_amount_to_receive_squared
        );
    }

    #[tokio::test]
    async fn previews_adding_b_when_liquidity_is_zero() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;

        let amount_b = 100;
        let expected_a_to_add = amount_b;
        let expected_liquidity_asset_amount_to_receive_squared = amount_b * expected_a_to_add;

        let preview = preview_add_liquidity(
            &exchange.contract,
            CallParameters::default(),
            TxParameters::default(),
            amount_b,
            AssetId::new(*exchange.asset_b_id),
        )
        .await
        .value;

        assert_eq!(preview.other_asset_amount_to_add, expected_a_to_add);
        assert_eq!(
            preview.liquidity_asset_amount_to_receive * preview.liquidity_asset_amount_to_receive,
            expected_liquidity_asset_amount_to_receive_squared
        );
    }

    #[tokio::test]
    async fn previews_adding_a_when_liquidity_is_not_zero_based_on_a() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 400;
        let initial_liquidity = 200;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            initial_liquidity,
            1000,
        )
        .await;

        let liquidity = pool_info(&exchange.contract).await.value.liquidity;

        let amount_a = 100;
        let expected_b_to_add = amount_a * deposit_amount_b / deposit_amount_a;
        let expected_liquidity_asset_amount_to_receive = amount_a * liquidity / deposit_amount_a;

        let preview = preview_add_liquidity(
            &exchange.contract,
            CallParameters::default(),
            TxParameters {
                gas_price: 0,
                gas_limit: 10_000_000,
                maturity: 0,
            },
            amount_a,
            AssetId::new(*exchange.asset_a_id),
        )
        .await
        .value;

        assert_eq!(preview.other_asset_amount_to_add, expected_b_to_add);
        assert_eq!(
            preview.liquidity_asset_amount_to_receive,
            expected_liquidity_asset_amount_to_receive
        );
    }

    #[tokio::test]
    async fn previews_adding_a_when_liquidity_is_not_zero_based_on_b() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 400;
        let deposit_amount_b = 100;
        let initial_liquidity = 200;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            initial_liquidity,
            1000,
        )
        .await;

        let liquidity = pool_info(&exchange.contract).await.value.liquidity;

        let amount_a = 100;
        let expected_b_to_add = amount_a * deposit_amount_b / deposit_amount_a;
        let expected_liquidity_asset_amount_to_receive =
            expected_b_to_add * liquidity / deposit_amount_b;

        let preview = preview_add_liquidity(
            &exchange.contract,
            CallParameters::default(),
            TxParameters {
                gas_price: 0,
                gas_limit: 10_000_000,
                maturity: 0,
            },
            amount_a,
            AssetId::new(*exchange.asset_a_id),
        )
        .await
        .value;

        assert_eq!(preview.other_asset_amount_to_add, expected_b_to_add);
        assert_eq!(
            preview.liquidity_asset_amount_to_receive,
            expected_liquidity_asset_amount_to_receive
        );
    }

    #[tokio::test]
    async fn previews_adding_b_when_liquidity_is_not_zero_based_on_a() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 400;
        let initial_liquidity = 200;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            initial_liquidity,
            1000,
        )
        .await;

        let liquidity = pool_info(&exchange.contract).await.value.liquidity;

        let amount_b = 100;
        let expected_a_to_add = amount_b * deposit_amount_a / deposit_amount_b;
        let expected_liquidity_asset_amount_to_receive =
            expected_a_to_add * liquidity / deposit_amount_a;

        let preview = preview_add_liquidity(
            &exchange.contract,
            CallParameters::default(),
            TxParameters {
                gas_price: 0,
                gas_limit: 10_000_000,
                maturity: 0,
            },
            amount_b,
            AssetId::new(*exchange.asset_b_id),
        )
        .await
        .value;

        assert_eq!(preview.other_asset_amount_to_add, expected_a_to_add);
        assert_eq!(
            preview.liquidity_asset_amount_to_receive,
            expected_liquidity_asset_amount_to_receive
        );
    }

    #[tokio::test]
    async fn previews_adding_b_when_liquidity_is_not_zero_based_on_b() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 400;
        let deposit_amount_b = 100;
        let initial_liquidity = 200;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            initial_liquidity,
            1000,
        )
        .await;

        let liquidity = pool_info(&exchange.contract).await.value.liquidity;

        let amount_b = 100;
        let expected_a_to_add = amount_b * deposit_amount_a / deposit_amount_b;
        let expected_liquidity_asset_amount_to_receive = amount_b * liquidity / deposit_amount_b;

        let preview = preview_add_liquidity(
            &exchange.contract,
            CallParameters::default(),
            TxParameters {
                gas_price: 0,
                gas_limit: 10_000_000,
                maturity: 0,
            },
            amount_b,
            AssetId::new(*exchange.asset_b_id),
        )
        .await
        .value;

        assert_eq!(preview.other_asset_amount_to_add, expected_a_to_add);
        assert_eq!(
            preview.liquidity_asset_amount_to_receive,
            expected_liquidity_asset_amount_to_receive
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
            CallParameters::default(),
            TxParameters::default(),
            100,
            AssetId::new(*asset_a_id),
        )
        .await;
    }
}
