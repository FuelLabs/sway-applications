use crate::utils::{
    abi_calls::{asset_balance, transfer_asset_to_output},
    test_helpers::{build_contract, setup_and_initialize},
    Identity,
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn can_transfer_coins() {
        let (owner, .., asset_instance) = setup_and_initialize().await;

        let wallet_native_balance_before = owner.get_asset_balance(&BASE_ASSET_ID).await.unwrap();

        let send_native_asset_amount = 100;

        // Send native assets to the contract
        let call_params = CallParameters::new(Some(send_native_asset_amount), None, None);
        let contract_native_asset_balance = asset_balance(
            &asset_instance,
            call_params,
            ContractId::from(*BASE_ASSET_ID),
        )
        .await;

        assert_eq!(contract_native_asset_balance, send_native_asset_amount);

        transfer_asset_to_output(
            &asset_instance,
            ContractId::from(*BASE_ASSET_ID),
            send_native_asset_amount,
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
    async fn on_non_owner_transfer_asset_to_output() {
        let (owner, minter, mint_amount, asset_contract_id, _asset_instance) =
            setup_and_initialize().await;

        let asset_instance_alternative =
            build_contract(asset_contract_id.clone(), minter.clone()).await;

        let address = Address::from(owner.address());

        transfer_asset_to_output(
            &asset_instance_alternative,
            ContractId::from(*asset_contract_id.hash()),
            mint_amount,
            Identity::Address(Address::from(address.clone())),
        )
        .await;
    }
}
