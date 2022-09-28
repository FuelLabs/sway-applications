use crate::utils::{
    abi_calls::token_balance,
    test_helpers::{build_contract, setup_and_initialize},
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn owner_can_get_token_balance() {
        let (.., token_instance) = setup_and_initialize().await;
        let send_native_token_amount = 100;

        let call_params = CallParameters::new(Some(send_native_token_amount), None, None);
        let contract_native_token_balance = token_balance(
            &token_instance,
            call_params,
            ContractId::from(*BASE_ASSET_ID),
        )
        .await;

        assert_eq!(contract_native_token_balance, send_native_token_amount);
    }

    #[tokio::test]
    async fn non_owner_can_get_token_balance() {
        let (_owner, minter, _mint_amount, token_contract_id, _token_instance) =
            setup_and_initialize().await;
        let send_native_token_amount = 100;

        let token_instance_alternative =
            build_contract(token_contract_id.clone(), minter.clone()).await;

        let call_params = CallParameters::new(Some(send_native_token_amount), None, None);
        let contract_native_token_balance = token_balance(
            &token_instance_alternative,
            call_params,
            ContractId::from(*BASE_ASSET_ID),
        )
        .await;

        assert_eq!(contract_native_token_balance, send_native_token_amount);
    }
}
