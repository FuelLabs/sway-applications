use crate::utils::{
    abi_calls::{balance, mint_coins},
    test_helpers::{build_contract, setup_and_initialize},
};

mod success {
    use super::*;

    #[tokio::test]
    async fn can_get_balance() {
        let (_owner, minter, mint_amount, asset_contract_id, asset_instance) =
            setup_and_initialize().await;

        mint_coins(&asset_instance, mint_amount).await;

        let asset_instance_alternative =
            build_contract(asset_contract_id.clone(), minter.clone()).await;

        assert_eq!(balance(&asset_instance_alternative).await, mint_amount);
    }
}
