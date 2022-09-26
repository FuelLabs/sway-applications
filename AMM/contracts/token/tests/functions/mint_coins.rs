use crate::utils::{
    abi_calls::{get_balance, initialize, mint_coins},
    test_helpers::{build_contract, setup},
    Identity,
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn can_mint_tokens() {
        let (owner, .., token_instance) = setup().await;
        let mint_amount = 10000;
        initialize(
            &token_instance,
            Identity::Address(Address::from(owner.address())),
            mint_amount,
        )
        .await;
        mint_coins(&token_instance, mint_amount).await;
        let balance = get_balance(&token_instance).await;
        assert_eq!(balance, mint_amount);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_non_owner_mint_coins() {
        let (owner, minter, .., token_contract_id, token_instance) = setup().await;
        let mint_amount = 10000;
        initialize(
            &token_instance,
            Identity::Address(Address::from(owner.address())),
            mint_amount,
        )
        .await;
        let token_instance_alternative =
            build_contract(token_contract_id.clone(), minter.clone()).await;
        mint_coins(&token_instance_alternative, mint_amount).await;
    }
}
