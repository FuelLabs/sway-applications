use crate::utils::{
    abi_calls::preview_swap_with_minimum,
    test_helpers::{deposit_and_add_liquidity, setup, setup_and_initialize},
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn previews_partial_swap_of_a() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 200;
        let swap_amount = 10;

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

        let preview_swap_info = preview_swap_with_minimum(
            &exchange.contract,
            CallParameters::new(None, Some(AssetId::new(*exchange.asset_a_id)), None),
            swap_amount,
        )
        .await
        .value;

        // hardcoded calculation for liquidity miner fee of 333
        let expected_amount = (swap_amount * (1 - (1 / 333)) * deposit_amount_b)
            / (deposit_amount_a + (swap_amount * (1 - (1 / 333))));
        let expected_reserve_depleted = !(expected_amount < deposit_amount_b);

        assert_eq!(preview_swap_info.amount, expected_amount);
        assert_eq!(
            preview_swap_info.reserve_depleted,
            expected_reserve_depleted
        );
    }

    #[tokio::test]
    async fn previews_partial_swap_of_b() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 200;
        let swap_amount = 10;

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

        let preview_swap_info = preview_swap_with_minimum(
            &exchange.contract,
            CallParameters::new(None, Some(AssetId::new(*exchange.asset_b_id)), None),
            swap_amount,
        )
        .await
        .value;

        // hardcoded calculation for liquidity miner fee of 333
        let expected_amount = (swap_amount * (1 - (1 / 333)) * deposit_amount_a)
            / (deposit_amount_b + (swap_amount * (1 - (1 / 333))));
        let expected_reserve_depleted = !(expected_amount < deposit_amount_a);

        assert_eq!(preview_swap_info.amount, expected_amount);
        assert_eq!(
            preview_swap_info.reserve_depleted,
            expected_reserve_depleted
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

        preview_swap_with_minimum(
            &exchange_instance,
            CallParameters::new(None, Some(AssetId::new(*asset_a_id)), None),
            10,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_msg_asset_id_is_invalid() {
        let (exchange, _wallet, asset_c_id) = setup_and_initialize().await;

        preview_swap_with_minimum(
            &exchange.contract,
            // sending invalid asset
            CallParameters::new(None, Some(AssetId::new(*asset_c_id)), None),
            10,
        )
        .await;
    }
}
