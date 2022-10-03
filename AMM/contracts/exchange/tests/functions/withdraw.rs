use crate::utils::{
    abi_calls::{balance, deposit, withdraw},
    test_helpers::setup_and_initialize,
};
use fuels::{prelude::*, tx::ContractId};

mod success {
    use super::*;

    #[tokio::test]
    async fn can_withdraw_base_asset() {
        let (
            exchange_instance,
            _wallet,
            _pool_asset_id,
            base_asset_id,
            _other_asset_id,
            _invalid_asset_id,
        ) = setup_and_initialize().await;

        let base_amount = 100;

        let call_params = CallParameters::new(Some(base_amount), None, None);
        deposit(&exchange_instance, call_params).await;
        withdraw(
            &exchange_instance,
            base_amount,
            ContractId::from(*base_asset_id),
        )
        .await;

        let balance = balance(&exchange_instance, ContractId::from(*base_asset_id)).await;
        assert_eq!(balance, 0);
    }

    #[tokio::test]
    async fn can_withdraw_other_asset() {
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
        withdraw(
            &exchange_instance,
            other_amount,
            ContractId::from(*other_asset_id),
        )
        .await;

        let balance = balance(&exchange_instance, ContractId::from(*other_asset_id)).await;
        assert_eq!(balance, 0);
    }
}
