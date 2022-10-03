use crate::utils::{
    abi_calls::balance,
    test_helpers::{setup, setup_and_initialize},
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn can_get_balance() {
        let (
            exchange_instance,
            _wallet,
            _pool_asset_id,
            base_asset_id,
            _other_asset_id,
            _invalid_asset_id,
        ) = setup_and_initialize().await;
        let balance = balance(&exchange_instance, ContractId::from(*base_asset_id)).await;

        assert_eq!(balance, 0);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "InvalidData")]
    async fn when_not_initialized() {
        // call setup instead of setup_and_initialize
        let (
            exchange_instance,
            _wallet,
            _pool_asset_id,
            base_asset_id,
            _other_asset_id,
            _invalid_asset_id,
        ) = setup().await;

        balance(&exchange_instance, ContractId::from(*base_asset_id)).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidData")]
    async fn when_asset_invalid() {
        let (
            exchange_instance,
            _wallet,
            _pool_asset_id,
            _base_asset_id,
            _other_asset_id,
            invalid_asset_id,
        ) = setup_and_initialize().await;

        // send invalid asset id
        balance(&exchange_instance, ContractId::from(*invalid_asset_id)).await;
    }
}
