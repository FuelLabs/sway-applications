use crate::utils::{
    abi_calls::{balance, burn_coins, mint_coins},
    test_helpers::{build_contract, setup_and_initialize},
};

mod success {
    use super::*;

    #[tokio::test]
    async fn owner_can_burn_all_coins() {
        let (_owner, _minter, mint_amount, _token_contract_id, token_instance) =
            setup_and_initialize().await;

        mint_coins(&token_instance, mint_amount).await;
        burn_coins(&token_instance, mint_amount).await;

        assert_eq!(balance(&token_instance).await, 0);
    }

    #[tokio::test]
    async fn owner_can_burn_coins_partially() {
        let (_owner, _minter, mint_amount, _token_contract_id, token_instance) =
            setup_and_initialize().await;
        let burn_amount = 5000;

        mint_coins(&token_instance, mint_amount).await;
        burn_coins(&token_instance, burn_amount).await;

        assert_eq!(balance(&token_instance).await, mint_amount - burn_amount);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_non_owner_burn() {
        let (_owner, minter, mint_amount, token_contract_id, token_instance) =
            setup_and_initialize().await;

        mint_coins(&token_instance, mint_amount).await;

        let token_instance_alternative =
            build_contract(token_contract_id.clone(), minter.clone()).await;

        burn_coins(&token_instance_alternative, mint_amount).await;
    }
}
