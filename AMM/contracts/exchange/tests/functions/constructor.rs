use crate::utils::{
    abi_calls::{constructor, pool_info},
    test_helpers::setup,
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn constructs() {
        let (exchange_instance, _wallet, _pool_asset_id, asset_a_id, asset_b_id, _asset_c_id) =
            setup().await;

        constructor(&exchange_instance, (asset_a_id, asset_b_id)).await;
        let pool_info = pool_info(&exchange_instance).await.value;

        assert_eq!(pool_info.asset_a, ContractId::new(*asset_a_id));
        assert_eq!(pool_info.asset_b, ContractId::new(*asset_b_id));
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_reinitialized() {
        let (exchange_instance, _wallet, _pool_asset_id, asset_a_id, asset_b_id, _asset_c_id) =
            setup().await;

        constructor(&exchange_instance, (asset_a_id, asset_b_id)).await;
        constructor(&exchange_instance, (asset_a_id, asset_b_id)).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_assets_in_pair_are_identical() {
        let (exchange_instance, _wallet, _pool_asset_id, asset_a_id, _asset_b_id, _asset_c_id) =
            setup().await;

        constructor(&exchange_instance, (asset_a_id, asset_a_id)).await;
    }
}
