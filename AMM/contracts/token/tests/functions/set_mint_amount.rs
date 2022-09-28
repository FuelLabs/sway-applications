use crate::utils::{
    abi_calls::{mint_amount, set_mint_amount},
    test_helpers::{build_contract, setup_and_initialize},
};

mod success {
    use super::*;

    #[tokio::test]
    async fn can_set_mint_amount() {
        let (.., token_instance) = setup_and_initialize().await;
        let new_mint_amount = 1;
        set_mint_amount(&token_instance, new_mint_amount).await;
        let mint_amount = mint_amount(&token_instance).await;
        assert_eq!(mint_amount, new_mint_amount);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_non_owner_set_mint_amount() {
        let (_owner, minter, _mint_amount, token_contract_id, _token_instance) =
            setup_and_initialize().await;
        let token_instance_alternative =
            build_contract(token_contract_id.clone(), minter.clone()).await;
        let new_mint_amount = 1;
        set_mint_amount(&token_instance_alternative, new_mint_amount).await;
    }
}
