use test_utils::interface::exchange::preview_add_liquidity;

mod success {
    use super::*;
    use crate::utils::setup_and_construct;
    use test_utils::{
        data_structures::LiquidityParameters, setup::common::deposit_and_add_liquidity,
    };

    #[tokio::test]
    async fn previews_adding_a_when_liquidity_is_zero() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;
        let expected_b_to_add = liquidity_parameters.amounts.0;
        let expected_liquidity_asset_to_receive_squared =
            liquidity_parameters.amounts.0 * expected_b_to_add;

        let preview = preview_add_liquidity(
            &exchange.instance,
            liquidity_parameters.amounts.0,
            exchange.pair.0,
            false,
        )
        .await;

        assert_eq!(preview.other_asset_to_add.amount, expected_b_to_add);
        assert_eq!(
            preview.liquidity_asset_to_receive.amount * preview.liquidity_asset_to_receive.amount,
            expected_liquidity_asset_to_receive_squared
        );
    }

    #[tokio::test]
    async fn previews_adding_b_when_liquidity_is_zero() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;
        let expected_a_to_add = liquidity_parameters.amounts.1;
        let expected_liquidity_asset_to_receive_squared =
            liquidity_parameters.amounts.1 * expected_a_to_add;

        let preview = preview_add_liquidity(
            &exchange.instance,
            liquidity_parameters.amounts.1,
            exchange.pair.0,
            false,
        )
        .await;

        assert_eq!(preview.other_asset_to_add.amount, expected_a_to_add);
        assert_eq!(
            preview.liquidity_asset_to_receive.amount * preview.liquidity_asset_to_receive.amount,
            expected_liquidity_asset_to_receive_squared
        );
    }

    #[tokio::test]
    async fn previews_adding_a_when_liquidity_is_not_zero_based_on_a() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let preview_amount_a = 100;
        let expected_b_to_add =
            preview_amount_a * liquidity_parameters.amounts.1 / liquidity_parameters.amounts.0;
        let expected_liquidity_asset_to_receive =
            preview_amount_a * liquidity_parameters.liquidity / liquidity_parameters.amounts.0;

        let preview =
            preview_add_liquidity(&exchange.instance, preview_amount_a, exchange.pair.0, true)
                .await;

        assert_eq!(preview.other_asset_to_add.amount, expected_b_to_add);
        assert_eq!(
            preview.liquidity_asset_to_receive.amount,
            expected_liquidity_asset_to_receive
        );
    }

    #[tokio::test]
    async fn previews_adding_a_when_liquidity_is_not_zero_based_on_b() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;

        let override_liquidity_parameters = LiquidityParameters::new(
            Some((
                liquidity_parameters.amounts.1,
                liquidity_parameters.amounts.0,
            )),
            Some(liquidity_parameters.deadline),
            Some(liquidity_parameters.liquidity),
        );

        let preview_amount_a = 100;
        let expected_b_to_add = preview_amount_a * override_liquidity_parameters.amounts.1
            / override_liquidity_parameters.amounts.0;
        let expected_liquidity_asset_to_receive = expected_b_to_add
            * override_liquidity_parameters.liquidity
            / override_liquidity_parameters.amounts.1;

        deposit_and_add_liquidity(&override_liquidity_parameters, &exchange, false).await;

        let preview =
            preview_add_liquidity(&exchange.instance, preview_amount_a, exchange.pair.0, true)
                .await;

        assert_eq!(preview.other_asset_to_add.amount, expected_b_to_add);
        assert_eq!(
            preview.liquidity_asset_to_receive.amount,
            expected_liquidity_asset_to_receive
        );
    }

    #[tokio::test]
    async fn previews_adding_b_when_liquidity_is_not_zero_based_on_a() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;

        let override_liquidity_parameters = LiquidityParameters::new(
            Some((
                liquidity_parameters.amounts.1,
                liquidity_parameters.amounts.0,
            )),
            Some(liquidity_parameters.deadline),
            Some(liquidity_parameters.liquidity),
        );

        let preview_amount_b = 100;
        let expected_a_to_add = preview_amount_b * override_liquidity_parameters.amounts.0
            / override_liquidity_parameters.amounts.1;
        let expected_liquidity_asset_to_receive = expected_a_to_add
            * override_liquidity_parameters.liquidity
            / override_liquidity_parameters.amounts.0;

        deposit_and_add_liquidity(&override_liquidity_parameters, &exchange, false).await;

        let preview =
            preview_add_liquidity(&exchange.instance, preview_amount_b, exchange.pair.1, true)
                .await;

        assert_eq!(preview.other_asset_to_add.amount, expected_a_to_add);
        assert_eq!(
            preview.liquidity_asset_to_receive.amount,
            expected_liquidity_asset_to_receive
        );
    }

    #[tokio::test]
    async fn previews_adding_b_when_liquidity_is_not_zero_based_on_b() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;

        let override_liquidity_parameters = LiquidityParameters::new(
            Some((
                liquidity_parameters.amounts.1,
                liquidity_parameters.amounts.0,
            )),
            Some(liquidity_parameters.deadline),
            Some(liquidity_parameters.liquidity),
        );

        let preview_amount_b = 100;
        let expected_a_to_add = preview_amount_b * override_liquidity_parameters.amounts.0
            / override_liquidity_parameters.amounts.1;
        let expected_liquidity_asset_to_receive = preview_amount_b
            * override_liquidity_parameters.liquidity
            / override_liquidity_parameters.amounts.1;

        deposit_and_add_liquidity(&override_liquidity_parameters, &exchange, false).await;

        let preview =
            preview_add_liquidity(&exchange.instance, preview_amount_b, exchange.pair.1, true)
                .await;

        assert_eq!(preview.other_asset_to_add.amount, expected_a_to_add);
        assert_eq!(
            preview.liquidity_asset_to_receive.amount,
            expected_liquidity_asset_to_receive
        );
    }
}

mod revert {
    use super::*;
    use crate::utils::setup;
    use fuels::prelude::AssetId;

    #[tokio::test]
    #[should_panic(expected = "AssetPairNotSet")]
    async fn when_uninitialized() {
        // call setup instead of setup_and_construct
        let (exchange_instance, _wallet, assets, _deadline) = setup().await;

        preview_add_liquidity(
            &exchange_instance,
            100,
            AssetId::new(*assets.asset_1),
            false,
        )
        .await;
    }
}
