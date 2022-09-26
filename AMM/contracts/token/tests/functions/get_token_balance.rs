use crate::utils::{
    abi_calls::{get_token_balance, initialize},
    test_helpers::{build_contract, setup},
    Identity,
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn owner_can_get_token_balance() {
        let (owner, .., token_instance) = setup().await;
        let mint_amount = 10000;
        let send_native_token_amount = 100;

        initialize(
            &token_instance,
            Identity::Address(Address::from(owner.address())),
            mint_amount,
        )
        .await;

        let call_params = CallParameters::new(Some(send_native_token_amount), None, None);
        let contract_native_token_balance = get_token_balance(
            &token_instance,
            call_params,
            ContractId::from(*BASE_ASSET_ID),
        )
        .await;

        assert_eq!(contract_native_token_balance, send_native_token_amount);
    }

    #[tokio::test]
    async fn non_owner_can_get_token_balance() {
        let (owner, not_owner, .., token_contract_id, token_instance) = setup().await;
        let mint_amount = 10000;
        let send_native_token_amount = 100;

        initialize(
            &token_instance,
            Identity::Address(Address::from(owner.address())),
            mint_amount,
        )
        .await;

        let token_instance_alternative =
            build_contract(token_contract_id.clone(), not_owner.clone()).await;

        let call_params = CallParameters::new(Some(send_native_token_amount), None, None);
        let contract_native_token_balance = get_token_balance(
            &token_instance_alternative,
            call_params,
            ContractId::from(*BASE_ASSET_ID),
        )
        .await;

        assert_eq!(contract_native_token_balance, send_native_token_amount);
    }
}
