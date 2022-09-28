use crate::utils::{
    abi_calls::{balance, mint_coins},
    test_helpers::{build_contract, setup_and_initialize},
};

mod success {
    use super::*;

    #[tokio::test]
    async fn can_mint_coins() {
        let (_owner, _minter, mint_amount, _asset_contract_id, asset_instance) =
            setup_and_initialize().await;
        mint_coins(&asset_instance, mint_amount).await;
        let balance = balance(&asset_instance).await;
        assert_eq!(balance, mint_amount);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_non_owner_mint_coins() {
        let (_owner, minter, mint_amount, asset_contract_id, _asset_instance) =
            setup_and_initialize().await;
        let asset_instance_alternative =
            build_contract(asset_contract_id.clone(), minter.clone()).await;
        mint_coins(&asset_instance_alternative, mint_amount).await;
    }
}
