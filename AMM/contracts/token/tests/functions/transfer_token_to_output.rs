use crate::utils::{
    abi_calls::{get_token_balance, transfer_token_to_output},
    test_helpers::{build_contract, setup_and_initialize},
    Identity,
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn can_transfer_coins() {
        let (owner, .., token_instance) = setup_and_initialize().await;

        let wallet_native_balance_before = owner.get_asset_balance(&BASE_ASSET_ID).await.unwrap();

        let send_native_token_amount = 100;

        // Send native tokens to the contract
        let call_params = CallParameters::new(Some(send_native_token_amount), None, None);
        let contract_native_token_balance = get_token_balance(
            &token_instance,
            call_params,
            ContractId::from(*BASE_ASSET_ID),
        )
        .await;

        assert_eq!(contract_native_token_balance, send_native_token_amount);

        transfer_token_to_output(
            &token_instance,
            ContractId::from(*BASE_ASSET_ID),
            send_native_token_amount,
            Identity::Address(Address::from(owner.address())),
        )
        .await;

        let wallet_native_balance_after = owner.get_asset_balance(&BASE_ASSET_ID).await.unwrap();
        assert_eq!(wallet_native_balance_before, wallet_native_balance_after);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_non_owner_transfer_token_to_output() {
        let (owner, minter, mint_amount, token_contract_id, _token_instance) =
            setup_and_initialize().await;

        let token_instance_alternative =
            build_contract(token_contract_id.clone(), minter.clone()).await;

        let address = Address::from(owner.address());

        transfer_token_to_output(
            &token_instance_alternative,
            ContractId::from(*token_contract_id.hash()),
            mint_amount,
            Identity::Address(Address::from(address.clone())),
        )
        .await;
    }
}
