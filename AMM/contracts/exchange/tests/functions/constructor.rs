use crate::utils::{
    abi_calls::{balance, constructor},
    test_helpers::setup,
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn initalizes() {
        let (
            exchange_instance,
            _wallet,
            _pool_asset_id,
            _base_asset_id,
            other_asset_id,
            _invalid_asset_id,
        ) = setup().await;

        constructor(&exchange_instance, ContractId::new(*other_asset_id)).await;

        let balance = balance(&exchange_instance, ContractId::from(*other_asset_id)).await;

        assert_eq!(balance, 0);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_already_initalized() {
        let (
            exchange_instance,
            _wallet,
            _pool_asset_id,
            _base_asset_id,
            other_asset_id,
            invalid_asset_id,
        ) = setup().await;

        constructor(&exchange_instance, ContractId::new(*other_asset_id)).await;
        constructor(&exchange_instance, ContractId::new(*invalid_asset_id)).await;
    }
}
