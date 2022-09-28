use crate::utils::{
    abi_calls::asset_balance,
    test_helpers::{build_contract, setup_and_initialize},
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn owner_can_get_asset_balance() {
        let (.., asset_instance) = setup_and_initialize().await;
        let send_native_asset_amount = 100;

        let call_params = CallParameters::new(Some(send_native_asset_amount), None, None);
        let contract_native_asset_balance = asset_balance(
            &asset_instance,
            call_params,
            ContractId::from(*BASE_ASSET_ID),
        )
        .await;

        assert_eq!(contract_native_asset_balance, send_native_asset_amount);
    }

    #[tokio::test]
    async fn non_owner_can_get_asset_balance() {
        let (_owner, minter, _mint_amount, asset_contract_id, _asset_instance) =
            setup_and_initialize().await;
        let send_native_asset_amount = 100;

        let asset_instance_alternative =
            build_contract(asset_contract_id.clone(), minter.clone()).await;

        let call_params = CallParameters::new(Some(send_native_asset_amount), None, None);
        let contract_native_asset_balance = asset_balance(
            &asset_instance_alternative,
            call_params,
            ContractId::from(*BASE_ASSET_ID),
        )
        .await;

        assert_eq!(contract_native_asset_balance, send_native_asset_amount);
    }
}
