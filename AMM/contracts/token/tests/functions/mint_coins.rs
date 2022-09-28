use crate::utils::{
    abi_calls::{balance, mint_coins},
    test_helpers::{build_contract, setup_and_initialize},
};

mod success {
    use super::*;

    #[tokio::test]
    async fn can_mint_tokens() {
        let (_owner, _minter, mint_amount, _token_contract_id, token_instance) =
            setup_and_initialize().await;
        mint_coins(&token_instance, mint_amount).await;
        let balance = balance(&token_instance).await;
        assert_eq!(balance, mint_amount);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_non_owner_mint_coins() {
        let (_owner, minter, mint_amount, token_contract_id, _token_instance) =
            setup_and_initialize().await;
        let token_instance_alternative =
            build_contract(token_contract_id.clone(), minter.clone()).await;
        mint_coins(&token_instance_alternative, mint_amount).await;
    }
}
