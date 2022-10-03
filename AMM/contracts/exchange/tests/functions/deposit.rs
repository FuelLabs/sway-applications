use crate::utils::{
    abi_calls::{balance, deposit},
    test_helpers::{setup, setup_and_initialize},
};
use fuels::{prelude::*, tx::ContractId};

mod success {
    use super::*;

    #[tokio::test]
    async fn can_deposit_base_asset() {
        let (
            exchange_instance,
            _wallet,
            _pool_asset_id,
            base_asset_id,
            _other_asset_id,
            _invalid_asset_id,
        ) = setup_and_initialize().await;
        let base_amount = 100;
        let call_params = CallParameters::new(Some(base_amount), Some(base_asset_id), None);

        deposit(&exchange_instance, call_params).await;

        let balance = balance(&exchange_instance, ContractId::from(*base_asset_id)).await;

        assert_eq!(balance, base_amount);
    }

    #[tokio::test]
    async fn can_deposit_other_asset() {
        let (
            exchange_instance,
            _wallet,
            _pool_asset_id,
            _base_asset_id,
            other_asset_id,
            _invalid_asset_id,
        ) = setup_and_initialize().await;
        let other_amount = 100;
        let call_params = CallParameters::new(Some(other_amount), Some(other_asset_id), None);

        deposit(&exchange_instance, call_params).await;

        let balance = balance(&exchange_instance, ContractId::from(*other_asset_id)).await;

        assert_eq!(balance, other_amount);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
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
        let amount = 100;

        let call_params = CallParameters::new(Some(amount), Some(base_asset_id), None);
        deposit(&exchange_instance, call_params).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_asset_invalid() {
        let (
            exchange_instance,
            _wallet,
            _pool_asset_id,
            _base_asset_id,
            _other_asset_id,
            invalid_asset_id,
        ) = setup_and_initialize().await;
        let amount = 1;

        // send invalid asset id
        let call_params = CallParameters::new(Some(amount), Some(invalid_asset_id), None);
        deposit(&exchange_instance, call_params).await;
    }
}
