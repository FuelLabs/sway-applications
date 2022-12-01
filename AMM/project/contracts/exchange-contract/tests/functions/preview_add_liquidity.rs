use crate::utils::{
    abi_calls::preview_add_liquidity,
    test_helpers::{
        deposit_and_add_liquidity, setup, setup_and_initialize,
        setup_initialize_deposit_and_add_liquidity,
    },
    MetaAmounts,
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn previews_adding_a_when_liquidity_is_zero() {
        let (exchange, _wallet, amounts, _asset_c_id) = setup_and_initialize().await;
        let expected_b_to_add = amounts.amount_a;
        let expected_liquidity_asset_amount_to_receive_squared =
            amounts.amount_a * expected_b_to_add;

        let preview = preview_add_liquidity(
            &exchange.instance,
            CallParameters::default(),
            TxParameters::default(),
            amounts.amount_a,
            exchange.asset_a,
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
        let (exchange, _wallet, amounts, _asset_c_id) = setup_and_initialize().await;
        let expected_a_to_add = amounts.amount_b;
        let expected_liquidity_asset_amount_to_receive_squared =
            amounts.amount_b * expected_a_to_add;

        let preview = preview_add_liquidity(
            &exchange.instance,
            CallParameters::default(),
            TxParameters::default(),
            amounts.amount_b,
            exchange.asset_a,
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
        let (exchange, _wallet, amounts, _asset_c_id, _added_liquidity) =
            setup_initialize_deposit_and_add_liquidity().await;

        let preview_amount_a = 100;
        let expected_b_to_add = preview_amount_a * amounts.amount_b / amounts.amount_a;
        let expected_liquidity_asset_amount_to_receive =
            preview_amount_a * amounts.liquidity / amounts.amount_a;

        let preview = preview_add_liquidity(
            &exchange.instance,
            CallParameters::default(),
            TxParameters {
                gas_price: 0,
                gas_limit: 100_000_000,
                maturity: 0,
            },
            preview_amount_a,
            exchange.asset_a,
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
        let (exchange, _wallet, amounts, _asset_c_id) = setup_and_initialize().await;

        let override_amounts = MetaAmounts {
            amount_a: 400,
            amount_b: 100,
            deadline: amounts.deadline,
            liquidity: amounts.liquidity,
        };
        let preview_amount_a = 100;
        let expected_b_to_add =
            preview_amount_a * override_amounts.amount_b / override_amounts.amount_a;
        let expected_liquidity_asset_amount_to_receive =
            expected_b_to_add * override_amounts.liquidity / override_amounts.amount_b;

        deposit_and_add_liquidity(&override_amounts, &exchange).await;

        let preview = preview_add_liquidity(
            &exchange.instance,
            CallParameters::default(),
            TxParameters {
                gas_price: 0,
                gas_limit: 100_000_000,
                maturity: 0,
            },
            preview_amount_a,
            exchange.asset_a,
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
        let (exchange, _wallet, amounts, _asset_c_id) = setup_and_initialize().await;

        let override_amounts = MetaAmounts {
            amount_a: 400,
            amount_b: 100,
            deadline: amounts.deadline,
            liquidity: amounts.liquidity,
        };
        let preview_amount_b = 100;
        let expected_a_to_add =
            preview_amount_b * override_amounts.amount_a / override_amounts.amount_b;
        let expected_liquidity_asset_amount_to_receive =
            expected_a_to_add * override_amounts.liquidity / override_amounts.amount_a;

        deposit_and_add_liquidity(&override_amounts, &exchange).await;

        let preview = preview_add_liquidity(
            &exchange.instance,
            CallParameters::default(),
            TxParameters {
                gas_price: 0,
                gas_limit: 100_000_000,
                maturity: 0,
            },
            preview_amount_b,
            exchange.asset_b,
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
        let (exchange, _wallet, amounts, _asset_c_id) = setup_and_initialize().await;

        let override_amounts = MetaAmounts {
            amount_a: 400,
            amount_b: 100,
            deadline: amounts.deadline,
            liquidity: amounts.liquidity,
        };
        let preview_amount_b = 100;
        let expected_a_to_add =
            preview_amount_b * override_amounts.amount_a / override_amounts.amount_b;
        let expected_liquidity_asset_amount_to_receive =
            preview_amount_b * override_amounts.liquidity / override_amounts.amount_b;

        deposit_and_add_liquidity(&override_amounts, &exchange).await;

        let preview = preview_add_liquidity(
            &exchange.instance,
            CallParameters::default(),
            TxParameters {
                gas_price: 0,
                gas_limit: 100_000_000,
                maturity: 0,
            },
            preview_amount_b,
            exchange.asset_b,
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
    #[should_panic(expected = "Revert(18446744073709486080)")]
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
